<!--
This changelog follows a very particular format. 

Only the title 'changelog' may begin with 1 pound symbol '#'. 

Every version partition must begin with 2 pound symbols '##'. 

Any section under a version must begin wtih 3 pound symbols '###'. 

This is important for the auto-changelog extraction occuring during the CI/CD 
pipeline to list only the current verion's changes with every release. 

Add `- unreleased` along the next future version to prevent CI/CD from triggering release mechanism.
-->

# Changelog

## 0.12.0

### Features
- Adds `--force` to build command to skip checking for blueprint to exist
- Adds `.orbitignore` file for `new` and `init` commands with the build directory listed in case the ip does not have version control
- Adds `language-mode` setting under `[general]` for supporting VHDL, verilog, or mixed language projects
- Allows errors during parsing to be identified by source code file and then adds ability to generate an erroneous blueprint by using `--force` with `plan` command
- Adds `public` field to manifest, which explicitly list the hdl files which will be public to other projects that use the ip as a dependency. If no `public` field exists, then the default is to have all hdl files be public
- Adds visibility to HDL files (public, protected, private)
- `remove` command now has ability to erase ip from downloads

### Changes
- Improves errors for loading configuration files
- Changes `summary` field to `description` field for manifest files and config files
- Changes `details` field to `explanation` field for plugin and protocol entries
- Adds `public` field in Orbit.toml in place to deprecate .orbitpub files
- Install command now tries to update lockfile if it exists but is out of date with current manifest
- Renames docker images to more memorable names and defines clearer organization
- Updates version table to display state of each version for the specific ip
- Default urls for the `source` field in manifest files can now use `orbit.ip.name` and `orbit.ip.version` variables for variable substitution
- Adds checks for user-managed ip (development status) does not include reserved files (files starting with `.orbit-`)
- swaps `uninstall` command for new `remove` command to handle removing ip from cache as well as downloads

### Bug Fixes
- Fixes issue with building ip file list not respecting .orbitpub and forgetting to hide private design units of dependency ip
- Fixes issue with install not finding uuid of existing ip from downloads when reinstalling
- Fixes issue with .orbitpub not being kept during download of an ip when zipped into archive
- Fixes how orbit retrieves the current project's uuid in lockfile by only looking at name and the missing checksum
- Fixes error during install to continue install even though there already exists an ip with same name and version in cache or downloads without `--force` present

## 0.11.0

### Changes
- Updates links to use proper GitHub username related to `orbit` remote repository URL

## 0.10.1

### Features
- adds modelsim docker image with `orbit`
- display protocols and their definitions using `--list` with the `install` command

### Changes
- starts plugins and protocol processes from expected directories
- adds asterisk to ip in catalog that have a possible update
- improves errors for install when using a path to search
- improves implementation for download process when using `install` command
- adds documentation

## 0.10.0

### Changes
- denies unknown fields to the `[general]` and `[vhdl-format]` config tables

### Fixes
- fixes bug with topological sort during `plan` command when transforming from design units to file paths

## 0.9.8

### Features
- `[vhdl-format]` is now a new supported entry in `config.toml` configuration files- use this to define how to format VHDL code when fetching the next entity to instantiate (see documentation for more details [here](https://cdotrus.github.io/orbit/reference/configuration.html#the-vhdl-format-section))
- adds `build-dir` field to the `[general]` section in `config.toml`

### Changes
- `[[plugin]]` renames 'alias' entry to 'name' to be more consistent across other settings

### Fixes
- fixes configuration file precedence issue with some fields being incorrectly overridden by files lower in the precedence

## 0.9.7

### Features
- adds `--json` flag to 'get' command to export entity data as json

### Documentation
- Adds 'env' command manual page
- Revises manual pages
- Revises 'build' command manual
- Revises 'plan' command manual
- Adds quick help to manual config file
- Adds page for 'tree' command

## 0.9.6

### Changes
- Improves priority of versions being displayed for search command
- Allows install command to fetch from --url and installs are required dependencies
- Improves install command to install from catalog or path
- Enhances 'read' command with comment fetching for valid vhdl tokens
- Fixes bug with dependency references missing when 'work' is used on external library
- Adds new fields and sections to manifest: metadata, readme, authors

### Documentation
- Adds page for 'get' command
- Adds page for 'read' command
- Adds page for 'search' command
- Adds typical command flow to commands page
- Improves docs tooling with synchronization script
- Adds manual for 'show' command
- Updates glossary terms
- Removes stale documentation
- Updates installing page
- Updates DST topics page
- Updates documentation layout and information

## 0.9.5

### Changes
- uninstalls dynamic variants from cache when uninstalling original
- adds auto-repair function to download files for future upkeep
- improves installing and downloading and catalog usage for downloads
- adds '--limit' to search and uses starts_with comparison for ip lookup during search
- checks download slot before downloading to prevent unncessary calls
- allows downloads to appear in search command
- adds downloads/ folder for stashing snapshots of versions as compressed files
- adds 'tag' field for source to define extra data for protocol

### Fixes
- trims whitespace around `.` and `'` in "get" command's VHDL code
- resolves issue with entering/exiting subprograms and missing dependencies after
- resolves issue with tree not detecting dependencies in generate statements
- fixes bug avoiding merging dependencies among files before removing duplicates in blueprint

## 0.9.4

### Changes
- changes lockfile to store checksum under "checksum" field
- adds '--all' flag to "tree" command
- refactors "read" command
- adds version number to lock file for future compatibility
- fixes "tree" command to only list nodes within local ip graph
- allows variable substitution to custom protocol arguments
- introduces CACHEDIR.TAG file to the cache directory

### Documentation
- adds page about protocols

### Internal
- removes deprecated source code
- reorganizes command modules
- adds new script for generating synchronous command documentation across github pages and manual pages

## 0.9.3

### Changes
- removes checksum from root ip in lock file and improves search
- adds option for installing dev-deps during install or to ignore
- adds support for dev-dependencies
- refactors catalog accessor
- allows uninstall to specify version through ip spec arg
- removes ability to specify deprecated dev version as partial version
- refactors cli option for ip to accept spec
- improves search command filtering and output
- install all missing deps during install and enhances checksum review
- changes ip spec to use ':' delimiter
- flattens source map in lockfile for more compact view

## 0.9.2

### Changes
- fixes bug regarding global config.toml not being able to properly be created when not existing

## 0.9.1

### Changes
- supports `summary` field to Orbit.toml
- supports `summary` and `details` fields for protocols in config.toml
- automatically downloads missing dependencies from lock file during plan command
- refactors config command
- denies unknown fields to certain TOML tables
- allows protocols to be defined as in-line tables in Orbit.toml
- fixes bug with forcing local config to exist per project
- refactors context and multi-layer configs
- improves error messages for toml parsing
- fixes --all option on plan command to ignore terminal nodes of graph found in dependency packages
- removes duplicate entries in blueprint

## 0.9.0

### Changes
- major overhaul to all commands
- adds download command
- refactors backend IP filesystem management methodology
- new lockfiles
- new manifests
- and more to be further documented

### Fixes
- resolves issue with a function not having to use 'end function' syntax and breaking AST during VHDL parsing

## 0.8.8

### Fixes
- resolves error with windows installer script regarding `The system cannot find the file specified. (os error 2)`

## 0.8.7

### Fixes
- resolves issue with some function bodies in architecture declarations losing track of architecture body to catch dependencies from instantiations (test: `playground_fn_in_arch_dec`)
- avoids panic on `.unwrap()` for assuming architecture owner exists and will silently skip analyzing architecture during graphing if owner is not existing

## 0.8.6

### Fixes
- reads from table 'env' not '.env' in config file

## 0.8.5

### Fixes
- prints missing double quotes on string token for orbit get

## 0.8.4

### Features
- adds colored syntax to `orbit get` VHDL output
- displays Windows OS-specific env var with `orbit env`
- major readability improvements and additional checks against the lock file during `orbit launch`
- saves installation size to published manifests

### Changes
- displays `library <identifier>;` vhdl code segment when using `orbit get` with `-i` and no `-c`
- swaps `\` for `/` in filepaths for env variables in `orbit env`

## 0.8.3

### Features
- new entry for plugins in config.toml: `details`- accepts a string (typically will be a multi-line string) that a developer can provide to outline the details of the plugin
- can now view plugin data in `orbit build` and `orbit plan`: `orbit plan --plugin <alias> --list` to view filesets, the command, summary, details, and its root directory
- `orbit plan --all` will not error on ambiguous top/bench units

## 0.8.2

### Fixes
- `orbit build` on windows OS by default tries to read .bat files from PATH when a command is entered without an extension and is not initially found as .exe. To disable this behavior, set an environment variable called `ORBIT_WIN_LITERAL_CMD`.

## 0.8.1

### Changes
- black box entities in `orbit tree` display as colored yellow
- adds `--lock-only` flag to `orbit plan` to only generate a lockfile
- prints 'None' for dependencies if ip has zero dependencies during `orbit probe`
- saves installation size to ip's manifest when publishing to vendor for later probing helpful information
- adds error if units were not saved to manifest at vendor/availalble level and trying to probe them from that ip catalog level with `orbit probe`

### Fixes
- tree command no longer incorrectly auto-detects dependency units as roots
- installation by --path now behaves properly

### Documentation
- updates general docs

## 0.8.0

### Features
- adds black box entity instances in `orbit tree` when cannot identify a source code file for an instantiated entity
- allows `--build-dir` to be set on `orbit build` (should match a build dir previously used on `orbit plan`)
- `orbit edit` can now edit global config.toml with `--config`
- adds a new variable during template importing: `orbit.filename` corresponds to the filestem of the current file undergoing variable substitution
- allows paths to be copied as a new ip in `orbit new` with `--from` option

### Changes
- refactors `orbit new` to allow for creating files within current working ip
- refactors `orbit edit` to support different modes
- refactors `orbit read` to support different modes and cleaning of temporary directory
- orders plugin list by alphabetical naming according to alias
- improves manifest error messages for bad parsing

### Fixes
- tree avoids displaying package unit when referencing entity from package in vhdl instantiation statement

### Documentation
- updates readme command overview
- updates manual page descriptions
- adds read command long help text
- updates pkgid page to reflect current implementation

## 0.7.0

### Changes
- `orbit get` now requires `--add` flag to add ip as dependency to Orbit.toml `[dependencies]` table
- `adds file positions to duplicate identifier error`
- refactors `orbit get` syntax to avoid awkward colon usage
- bypasses DEV_PATH existence and directory check on `orbit config`
- adds rough implementation for `orbit read`
- adds error for direct dependency identifier naming conflict with current ip

### Documentation
- adds .orbitignore docs
- adds command overview
- updates search man page
- fixes command man page formatting

## 0.6.1

### Fixes
- properly detects entity name in instantiation code when referenced from package

## 0.6.0

### Features
- automatically installs all dependent ip from reading available lockfile detected during `orbit install`
- adds ip graph verification + duplicate identifier detection to `orbit launch` process
- supports ignoring certain files while still having version control over them with a `.orbitignore` file
- adds enviornment variable ORBIT_BLUEPRINT = "blueprint.tsv" on `orbit build`
- pulls latest to the repository store if an unknown version was provided during install and installing from internal store

### Changes
- renames `--ver, -v <version>` option to `--variant, -v <version>`

### Fixes
- `orbit get` with `--architecture` will now correctly display all known architectures for the entity in the current project
- issues error on duplicate primary design unit identifiers in same project

## 0.5.3

### Fixes
- `orbit plan` correctly searches current ip's files for fileset collection

## 0.5.2

### Features
- supports VHDL package generics and package instantiations

### Changes
- alters long formatting for `orbit tree` to be `<ip>:<entity> v<version>`
- refactors graphing to only need to be built once during `orbit plan` and `orbit tree`
- improves reference identifier detection in entity declarations and packages
- improves ip graph building against false edge detection by checking if library names match from references

## 0.5.1

### Features
- checks store for available versions of an ip during `orbit probe --tags`
- improves installing dependencies from lockfile
- displays warning when IP is initialized outside of DEV_PATH

### Fixes
- stores repos while maintaining origin remote
- tweaks VHDL code formatting output on `orbit get`
- improves filesystem copying operation with respecting .gitignore
- returns error if ip listed more than once as direct dependency in current project

### Documentation
- adds concept overview page
- updates orbit install example
- updates env var page
- adds topic page about dst

## 0.5.0

### Features
- multiple versions of ip are allowed to be in the build graph* due to dynamic symbol transformation algorithm

*an ip being used as two different versions is not allowed when both are direct dependencies to the current project

## 0.4.0

### Features
- adds support for detecting and connecting VHDL context design units in `orbit plan` for blueprint writing
- adds support for detecting and connecting VHDL configuration design units in `orbit plan` for blueprint writing

## 0.3.14

### Features
- adds `--force` flag to `orbit plan` to skip reading a lock file

## 0.3.13

### Features
- adds support for publishing ip to vendors during `orbit launch`
- vendor pre-publish and post-publish hooks
- caches primary design unit data on ip when publishing to vendor
- displays more env variables on `orbit env` when in an ip

### Fixes
- detecting manifests in vendors collects all in single directory

### Documentation
- updates vendors topics regarding hooks and configuration

## 0.3.12

### Features
- adds `--disable-ssh` flag to `orbit install`

### Fixes
- properly computes checksum during install and reverts to original cwd

## 0.3.11

### Features
- adds `--disable-ssh` flag to `orbit plan` to convert ssh connections to ssh in lock file
- displays direct dependencies in `orbit probe`
- adds `--ip` flag to `orbit tree` to view IP-level dependency tree
- implements basic vendor registry support in `config.toml`
- ip can be installed from available state

### Documentation
- updates configuration page for vendor entry
- updates vendors page

## 0.3.10

### Features
- adds `orbit uninstall` command to remove ip from catalog (development and installations)
- adds `--name` option to `orbit get` in setting the HDL instance's identifier

### Fixes
- verifies `--name` can only be paired with `--instance` on `orbit get`

### Documentation
- adds content to user guide pages

## 0.3.9

### Fixes
- properly writes toml comment to initial config.toml

## 0.3.8

### Changes
- adds computed and expected checksums during a failed upgrade with `--upgrade`

## 0.3.7

### Features
- `orbit env` displays environment variabels from config.toml
- supports variable replacement in fileset patterns during `orbit plan`
- displays architectures with --architecture flag in `orbit get`
- filters search results by accepted pkgid ip arg for `orbit search`

### Fixes
- resolves bugs when using --git and --path together for `orbit init`
- enhances array formatting when appending to include key using `orbit config`

### Documentation
- adds blueprint topic page
- adds entries to glossary
- adds disclaimer about license
- grammar revisions
- adds orbit.lock page
- begins first tutorials page

## 0.3.6

### Features
- prevents launch when dependency is from dev path
- a lockfile `Orbit.lock` is written during plan command
- reads `Orbit.lock` when current ip's lock entry matches manifest data to ensure all dependencies are pulled in

### Fixes
- url in blank `Orbit.toml` files points to the correct Orbit.toml help page

## 0.3.5

### Fixes
- Orbit.toml is no longer needed on git main branch of an ip to install

## 0.3.4

### Fixes
- installing with git works without having main branch being Orbit ip
- reading deps from Orbit.toml now correctly travels to all dependencies

## 0.3.3

### Features
- new config.toml entry: `core.user` (used for template variable `{{ orbit.user }}` )
- new config.toml entry: `core.date-fmt` (used for template variable `{{ orbit.date }}` )
- launch command now installs by default
- initial support for external dependencies in plan and tree commands
- tree command now supports `--format long` option to display full pkgid with every entity

### Fixes
- config --set now properly overwrites existing value
- entity direct instantiation now contains missing 'entity' keyword
- better error handling for toml deserialization

### Documentation
- updates config.toml entry references for date-fmt and user

## 0.3.2

### Fixes
- fixes edge-case checksum bugs

## 0.3.1

### Features
- adds --verbose flag to build command
- stores repositories into store/ to be used to checkout specific installation versions into cache

### Documentation
- revises develop ip page
- writes developing ip page
- writes template page
- updates initial setup and new command pages
- adds config man page to index
- updates configuration page

## 0.3.0

### Features
- `config` command: modify some configuration values directly from the command-line
- supports local config.toml files in current working ip
- templates: reusable directories with variable substitution support to import boilerplate files for new projects
- adds `--list` flag to new command to view usable templates

### Documentation
- updates initial setup and new command pages
- adds config man page to index
- updates configuration page

## 0.2.2

### Features
- the home config file (located at ORBIT_HOME) can now include other config files on your machine with the `include` key (value is a a list of strings)

## 0.2.1

### Features
- print environment information with `env` command
- allows `b` as shortcut to `build` command
- `plan` command now saves the plugin it was called with (if any). In this case the next future calls to `build` can opt out of specifying a plugin to default to the one used during the previous `plan`.

### Changes
- install accepts multiple methods to install from (--path, --git, --ip)
- `launch` checks if a remote repository is entered in the manifest if the git repository for that ip has one

### Fixes
- `probe --units` now displays units in alphabetical order
- `--list` for printing plugins now adds in newlines between every plugin

## 0.2.0

### Features
- view component declarations, signal declarations, and instantiation code using the `get` command
- version can be specified for `probe` command

### Changes
- `query` command is now the `probe` command- still functions the same
- improves interface with using the `install` command and entering arguments

### Fixes
- verifies Orbit.toml is not ignored by git
- adds nicer error message with suggestion when no installation version is found

### Documentation
- adds -m for launch and fixes man page formatting

## 0.1.8

### Changes
- improves ip manifest detection in filesystem traversal
- improves checksum generation and stores a local checksum file in its cache slot for internal reference

### Fixes
- `init` command works properly using existing paths and current working dir
- `build` command properly displays plugins with `--list`

### Documentation
- `init` manual accurately reflects its action

## 0.1.7

### Changes
- `launch` now checks against remote upstream branch to verify its in sync before approving
- `build` is forced to either have a plugin or command passed into cli to run

### Documentation
- adds first steps and plugins pages
- adds query command manual page

## 0.1.6

### Fixes
- copying directories now goes to correct level for `init` and `install` commands
- fixes issue with install failing on windows due to files being used by other processes

### Changes
- search command uses `--install, -i` rather than `--cache, -c` to show installed IP
- displays clearer error message if an IP is already installed

## 0.1.5

### Features
- new command: `init`- initialize existing projects as orbit IP

### Fixes
- users can now no longer create an IP with same pkgid as an already known IP from any level (dev, cache, available)

### Documentation
- adds repository key to orbit.toml page
- writes vendor page and adds blank user guides
- writes `init` man page

## 0.1.4

### Fixes
- improves vhdl symbol detection for parsing functions and procedures 
- resolves path issue for `get` command on current ip
- properly checks config.toml file with 'core' table before searching it
- ensures `core.editor` exists for edit command

### Documentation
- adds topics for ip and manifest

## 0.1.3

### Features
- enhances plugin display list using `orbit plan --list`
- supports graph building with in-order references to packages during `plan` command
- adds more progress messages to `--upgrade` action
- reads ip found in cache during `search` command

### Bug Fixes
- running `plan` with no `--top` and no `--bench` while having it auto-detect only a top does not result in an error
- nicer error mesage for identifier not found during `tree` command

## 0.1.2

### Features
- an installer program is included with the orbit project when downloading a precompiled binary

### Fixes
- windows-style endings (`\r\n`) supported when given interactive prompts

### Documentation
- adds a simple README included with the orbit project for distribution

## 0.1.1

### Bug Fixes
- `--upgrade` flag now properly searches for correct binary dependent on operating system during zip extraction

## 0.1.0

### Features
- implements basis for `new`, `edit`, `tree`, `plan`, `build`, `launch`, `search`, and `install` commands
- view current design heirarchy using `tree` command
- allows an option to be accepted multiple times on the command-line (example: `--filesets`)
- allows filesets to be created/overriden on command-line for `plan` command
- verifies `build` and `plan` commands are called from an "IP-directory-sensitive" path
- prevents IPs from becoming nested within each other's path in `new` command
- adds `build` command to execute an external process within orbit
- adds `plan` command to generate a blueprint.tsv file in a build output directory
- adds support for the toml configuration file format to store key value pairs
- creates home folder at ~/.orbit if `ORBIT_HOME` enviornment variable is not set
- adds `--upgrade` flag for self-updating binary with latest Github release for user's targeted OS and architecture
- adds command-line interface with helpful misspelling suggestions and argument input validation