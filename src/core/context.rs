use crate::core::config::General;
use crate::core::config::{Config, Configs, Locality};
use crate::core::plugin::Plugin;
use crate::util::anyerror::AnyError;
use crate::util::anyerror::Fault;
use crate::util::environment::ORBIT_WIN_LITERAL_CMD;
use crate::util::filesystem::Standardize;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::path;
use std::path::PathBuf;

use super::lang::LangMode;

const CACHE_TAG_FILE: &str = "CACHEDIR.TAG";

const CACHE_TAG: &str = "\
Signature: 8a477f597d28d172789f06886806bc55
# This file is a cache directory tag created by orbit.
# For information about cache directory tags see https://bford.info/cachedir/
";

/// Shared attributes about the surrounding user run-time environment.
pub struct Context {
    /// Path to a "hidden" directory for internal Orbit operations.
    home_path: PathBuf,
    /// Directory holding installed immutable tags of git repositories.
    cache_path: PathBuf,
    /// Directory holding orbit IP downloaded
    download_path: PathBuf,
    /// The parent path to the current ip `Orbit.toml` manifest file.
    ip_path: Option<PathBuf>,
    /// Directory name for the intermediate build processes and outputs.    
    build_dir: String,
    /// Language support mode.
    lang_mode: LangMode,
    /// Flattened view of the current configuration settings.
    config: Config,
    /// Entire list of configuration settings.
    all_configs: Configs,
    // @idea: optionally move hashmap out of context and create it from fn to allow dynamic loading
    plugins: HashMap<String, Plugin>,
}

impl Context {
    pub fn new() -> Context {
        let home = std::env::temp_dir();
        let cache = home.join("cache");
        let downloads = home.join("downloads");
        Context {
            home_path: home,
            cache_path: cache,
            download_path: downloads,
            ip_path: None,
            plugins: HashMap::new(),
            all_configs: Configs::new(),
            config: Config::new(),
            build_dir: String::new(),
            lang_mode: LangMode::default(),
        }
    }

    /// Sets the home directory. By default this is `$HOME/.orbit`. If set by `var`,
    /// it must be an existing directory.
    pub fn home(mut self, key: &str) -> Result<Context, ContextError> {
        self.home_path = if let Ok(s) = env::var(key) {
            std::path::PathBuf::from(s)
        } else {
            let hp = match home::home_dir() {
                Some(p) => p.join(".orbit"),
                None => return Err(ContextError(format!("failed to detect user's home directory; please set the ORBIT_HOME environment variable")))
            };
            // create the directory if does not exist
            if path::Path::exists(&hp) == false {
                std::fs::create_dir(&hp).expect("failed to create .orbit directory");
            }
            hp
        };
        // do not allow a non-existent directory to be set for the home
        if path::Path::exists(&self.home_path) == false {
            return Err(ContextError(format!(
                "directory {} does not exist for ORBIT_HOME",
                self.home_path.display()
            )));
        }
        // verify the environment variable is set
        env::set_var(key, &self.home_path);
        Ok(self)
    }

    /// Sets the cache directory. If it was set from `var`, it assumes the path
    /// exists. If setting by default (within HOME), it assumes HOME is already existing.
    pub fn cache(mut self, key: &str) -> Result<Context, Fault> {
        self.cache_path = self.folder(key, "cache")?;
        // create a cache tag file if does not exist
        match Self::is_cache_tag_valid(&self.cache_path) {
            Ok(_) => (),
            Err(e) => fs::write(&e, CACHE_TAG)?,
        }
        Ok(self)
    }

    /// Checks if the cache tag file is properly configured in the set cache directory.
    ///
    /// Returns an `Err` holding the path to the needed cache file if the path was
    /// not a file or did not exactly contain the content's string.
    pub fn is_cache_tag_valid(dir: &PathBuf) -> Result<(), PathBuf> {
        let tag = dir.join(CACHE_TAG_FILE);
        match tag.is_file() {
            false => Err(tag),
            true => match fs::read_to_string(&tag) {
                Err(_) => Err(tag),
                Ok(text) => match text == CACHE_TAG {
                    false => Err(tag),
                    true => Ok(()),
                },
            },
        }
    }

    /// Sets the downloads directory. If it was set from `var`, it assumes the path
    /// exists. If setting by default (within HOME), it assumes HOME is already existing.
    pub fn downloads(mut self, key: &str) -> Result<Context, Fault> {
        self.download_path = self.folder(key, "downloads")?;
        Ok(self)
    }

    /// Checks if windows literal command is enabled.
    pub fn enable_windows_bat_file_match() -> bool {
        if cfg!(target_os = "windows") {
            // by not finding the env var, the windows batch file match is enabled
            std::env::var(ORBIT_WIN_LITERAL_CMD).is_err()
        } else {
            false
        }
    }

    /// Returns an existing filesystem path to be used under `key`.
    ///
    /// Uses `key`'s value if already explicitly set and will set the environment
    /// variable accordingly.
    fn folder(&self, key: &str, folder: &str) -> Result<PathBuf, Fault> {
        // prioritize explicit variable setting
        let dir = if let Ok(s) = env::var(key) {
            let ep = PathBuf::from(s);
            // verify the path exists
            if ep.exists() == false {
                return Err(AnyError(format!(
                    "directory {} does not exist for {}",
                    ep.display(),
                    key
                )))?;
            }
            // verify the path is a directory
            if ep.is_dir() == false {
                return Err(AnyError(format!("{} must be a filesystem directory", key)))?;
            }
            ep
        // proceed with default
        } else {
            let ep = self.home_path.join(&folder);
            // create the directory if does not exist
            if ep.exists() == false {
                std::fs::create_dir(&ep)
                    .expect(&format!("failed to create .orbit/{} directory", folder));
            }
            ep
        };
        // set the environment variable
        env::set_var(key, &PathBuf::standardize(&dir));
        Ok(dir)
    }

    /// References the cache directory.
    pub fn get_cache_path(&self) -> &PathBuf {
        &self.cache_path
    }

    /// References the downloads directory
    pub fn get_downloads_path(&self) -> &PathBuf {
        &self.download_path
    }

    /// Configures and reads data from the settings object to return a `Settings` struct
    /// in the `Context`.
    ///
    /// The settings file `name` must be directly under `$ORBIT_HOME`. It also
    /// checks for a local configuration as `name` under a .orbit/ directory if
    /// the command is invoked from within an ip directory.
    ///
    /// Note: the `self.ip_path` must already be determined before invocation.
    pub fn settings(mut self, name: &str) -> Result<Context, Fault> {
        // check if global file exists first
        let global_file = self.home_path.join(name);
        if global_file.exists() == false {
            std::fs::write(&global_file, Vec::new())?;
        }

        // initialize and load the global configuration
        let cfg = Configs::new().load(global_file, Locality::Global)?;
        // if in ip, also look along current directory for a /.orbit/config.toml file to load (local configuration)
        self.all_configs = if let Some(ip_dir) = self.get_ip_path() {
            let local_path = ip_dir.join(".orbit").join(name);
            if local_path.exists() == true {
                cfg.load(local_path, Locality::Local)?
            } else {
                cfg
            }
        } else {
            cfg
        };

        // @TODO: FIXME (clone?)
        // initialize and load the global configuration
        let cfg = Configs::new().load(self.home_path.join(name), Locality::Global)?;
        // if in ip, also look along current directory for a /.orbit/config.toml file to load (local configuration)
        self.config = if let Some(ip_dir) = self.get_ip_path() {
            let local_path = ip_dir.join(".orbit").join(name);
            if local_path.exists() == true {
                cfg.load(local_path, Locality::Local)?
            } else {
                cfg
            }
        } else {
            cfg
        }
        .into();
        // @todo: dynamically set from environment variables from configuration data
        Ok(self)
    }

    /// Access the configuration data.
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    pub fn get_all_configs(&self) -> &Configs {
        &self.all_configs
    }

    /// Access the configuration data as mutable.
    pub fn get_config_mut(&mut self) -> &mut Config {
        &mut self.config
    }

    /// Access the build directory data.
    pub fn get_build_dir(&self) -> String {
        match self.config.get_general() {
            Some(g) => g.get_build_dir(),
            None => General::new().get_build_dir(),
        }
    }

    /// Access the language mode data.
    pub fn get_lang_mode(&self) -> LangMode {
        match self.config.get_general() {
            Some(g) => g.get_lang_mode(),
            None => General::new().get_lang_mode(),
        }
    }

    /// Access the ip directory detected from the current working directory.
    pub fn get_ip_path(&self) -> Option<&path::PathBuf> {
        self.ip_path.as_ref()
    }

    /// Access the home path.
    pub fn get_home_path(&self) -> &path::PathBuf {
        &self.home_path
    }

    /// Determines if the directory is within a current IP and sets the proper
    /// runtime environment variable.
    pub fn current_ip_dir(mut self, s: &str) -> Result<Context, ContextError> {
        self.ip_path = match Context::find_ip_path(
            &std::env::current_dir().expect("failed to get current directory"),
        ) {
            Some(cwd) => {
                env::set_var(s, &cwd);
                Some(cwd)
            }
            None => None,
        };
        Ok(self)
    }

    /// Changes current working directory to the detected IP path.
    ///
    /// Returns an error if ip_path is `None`.
    pub fn goto_ip_path(&self) -> Result<(), ContextError> {
        match self.get_ip_path() {
            Some(cwd) => {
                // set the current working directory to here
                std::env::set_current_dir(&cwd).expect("could not change directories");
            }
            None => {
                // @IDEA also give information about reading about ip-dir sensitive commands as a topic?
                return Err(ContextError(format!(
                    "no orbit IP detected in current directory or any parent directory"
                )));
            }
        }
        Ok(())
    }

    /// Finds the complete path to the current IP's directory.
    ///
    /// This function will recursively backtrack down the current working directory
    /// until finding the first directory with a file named "Orbit.toml".
    pub fn find_ip_path(dir: &std::path::PathBuf) -> Option<path::PathBuf> {
        Self::find_target_path(dir, "Orbit.toml")
    }

    /// Finds the complete path to the current directory that hosts the `target_file`.
    ///
    /// This function recursively backtracks from `dir` into its ancestors until
    /// finding the first directory with a file named `target_file`.
    ///
    /// This function has no assumptions on if the directory is readable or not (bypasses read_dir errors).
    pub fn find_target_path(dir: &std::path::PathBuf, target_file: &str) -> Option<path::PathBuf> {
        let mut cur = dir.clone();
        // search for the manifest file
        loop {
            match std::fs::read_dir(&cur) {
                // the directory was able to be read (it exists)
                Ok(mut entries) => {
                    let result = entries.find_map(|p| match p {
                        Ok(file) => {
                            if file.file_name() == target_file {
                                Some(cur.to_path_buf())
                            } else {
                                None
                            }
                        }
                        _ => None,
                    });
                    if let Some(r) = result {
                        break Some(r);
                    }
                }
                // failed to read the directory
                Err(_) => {}
            }
            if cur.pop() == false {
                break None;
            }
        }
    }

    /// Sets the IP's build directory and the corresponding environment variable.
    pub fn build_dir(self, s: &str) -> Result<Context, ContextError> {
        env::set_var(s, &self.get_build_dir());
        Ok(self)
    }
}

#[derive(Debug)]
pub struct ContextError(String);

impl std::error::Error for ContextError {}

impl std::fmt::Display for ContextError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const HOME: &str = "./tests/env";

    #[test]
    fn find_target_path() {
        // existing path with target at root
        let home = HOME.to_owned();
        let p = Context::find_target_path(&PathBuf::from(home.clone() + "/project1"), "Orbit.toml");
        assert_eq!(p, Some(PathBuf::from(home.clone() + "/project1")));

        // inner path with target a directory back
        let p = Context::find_target_path(&PathBuf::from("./src"), "Cargo.toml");
        assert_eq!(p, Some(PathBuf::from(".")));

        // imaginary path with target a couple directories back
        let p = Context::find_target_path(
            &PathBuf::from(home.clone() + "/project1/rtl/syn/"),
            "Orbit.toml",
        );
        assert_eq!(p, Some(PathBuf::from(home.clone() + "/project1")));

        // no existing target
        let p = Context::find_target_path(
            &PathBuf::from(home.clone() + "/project1/rtl/syn/"),
            "HIDDEN-TARGET.TXT",
        );
        assert_eq!(p, None);
    }
}
