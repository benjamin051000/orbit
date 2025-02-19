# __orbit build__

## __NAME__

build - execute a backend workflow

## __SYNOPSIS__

```
orbit build [options] [--] [args]...
```

## __DESCRIPTION__

This command carries out the "building phase". This phase involves running a
user-defined command or plugin as a subprocess. It is required that the
planning phase occurs before the building phase. This command acts upon the 
current working ip.

If a plugin was previously used during the planning phase, then this command
by default will reference and call that plugin after loading the previously
written `.env` file from the planning phase. Either a plugin from `--plugin` 
or a command from `--command` is required if a plugin was not previously
specified during planning.

If `--list` is used, then it will display a list of the available plugins to
the user. Using `--list` in combination with a plugin from `--plugin` will
display any detailed help information the plugin has documented in its 
definition.

As a refresher, a backend workflow typically performs three tasks:  
   1. Parse the blueprint file  
   2. Process the referenced files listed in the blueprint  
   3. Generate an output product 

Any command-line arguments entered after the terminating flag `--` will be
passed in the received order as arguments to the subprocess's command. If a plugin already
has defined arguments, the additional arguments passed from the command-line
will follow the previously defined arguments.

The subprocess will spawn from the current working ip's build directory.

## __OPTIONS__

`--plugin <name>`  
      Plugin to execute

`--command <cmd>`  
      Command to execute

`--list`  
      View available plugins

`--force`  
      Execute the command without checking for a blueprint

`--build-dir <dir>`  
      The relative directory to locate the blueprint file

`--verbose`  
      Display the command being executed

`args`  
      Arguments to pass to the plugin or command

## __EXAMPLES__

```
orbit build --plugin xsim -- --elab
orbit build --command python -- synth.py
orbit build --verbose
orbit build --plugin xsim --force -- --help
```

