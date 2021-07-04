# prog

 A tool for centralizing scripted shell commands via a configurable JSON file.

## Table of Contents

 1. [Downloading and Installing](#downloading-and-installing)
 2. [Usage](#usage)
 3. [Future Plans](future-plans)

## Downloading and Installing

#### Downloading

 prog is currently available as a GitHub repository, and can be cloned using the
 following command:
 `git clone https://github.com/bdreece/prog.git`

#### Installing

 prog is packaged via the setup script, and can be installed using the following
 command:
 `cd prog; pip install .`

 Congratulations! You should now have prog installed on your system!

## Usage

#### Help Page

 The help page for prog can be displayed in the following manner:
 `prog --help`

 This will display the following information:
 ```
 Usage: prog [OPTIONS] [COMMANDS]...

   A command line utility for centralizing scripted shell commands via a
   configurable JSON file

 Options:
   -g, --generate PATH  Generate default JSON file
   -v, --verbose        Show verbose output
   -f, --file PATH      Path to JSON file
   --version            Show the version and exit.
   --help               Show this message and exit.
 ```

#### JSON File

 A default JSON file can be generated using the `prog -g` command, optionally
 specifying an output file (default: `./prog.json`). This file is based on a
 C-style project, with commands listed for building, running, and debugging a
 project, and is configured as follows:
 ```
 {
   "build": "make all",
   "run": "./main",
   "debug": "gdb ./main"
 }
 ```
 The command hooks are listed in the keys of the JSON file, and their corresponding
 shell commands are the respective values of the JSON file.

#### Invocation

 Once the JSON file has been configured to your needs, you can simply invoke the
 custom commands using the following command:
 `prog [COMMANDS]...`

 Multiple commands may be specified, and will execute in the order they are specified.

## Future Plans

 As of right now, the prog command is about as simple as possible, while still
 operating as intended. I pictured this to be a higher-level solution than say
 a Makefile or other build system. One can orchestrate compilation and execution
 of project code through one centralized command.
