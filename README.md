# prog

[![](https://img.shields.io/pypi/wheel/prog-0.1.2)](https://pypi.org/project/prog-0.1.2/)

 A tool for centralizing scripted shell commands via a configurable JSON or YAML file.

## Table of Contents

 1. [Downloading and Installing](#downloading-and-installing)
 2. [Usage](#usage)
 3. [Future Plans](future-plans)

## Downloading and Installing

#### PyPI

 prog is available on PyPI! You can install it via `pip install prog-0.1.2`

#### Downloading

 prog is currently available as a GitHub repository, and can be cloned using the
 following command:
 `git clone https://github.com/bdreece/prog.git`

#### Installing

 prog is packaged via the setup script, and can be manually installed using the following
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
   configurable JSON or YAML file

 Options:
   -j, --json       Use JSON config file
   -y, --yml        Use YAML config file
   -g, --generate   Create a config file
   -e, --edit       Edit config file
   -v, --verbose    Show verbose output
   -f, --file PATH  Path to config file
   -V, --version    Show the version and exit.
   -h, --help       Show this message and exit.
 ```

#### JSON Config File

 A default JSON file can be generated using the `prog -jg` command, optionally specifying an output file with `prog -jgf <PATH>` (default: `./prog.json`). This file is based on a C-style project, with commands listed for building, running, and debugging a project, and is configured as follows:

 ```json
 {
   "build": "make all",
   "run": "./main",
   "debug": "gdb ./main"
 }
 ```

 The aliased command hooks are listed in the keys of the JSON file, and their corresponding shell commands are the respective values of the JSON file.

#### YAML Config File

 A default YAML file can be generated using the `prog -yg` command, optionally specifying an output file with `prog -ygf <PATH>` (default: `./prog.yml`). This file is based on a C-style project, with commands listed for building, running, and debugging a project, and is configured as follows:

```yaml
 build: make all
 run: ./main
 debug: gdb ./main
```

 The aliased command hooks are listed in the keys of the YAML file, and their corresponding shell commands are the respective values of the YAML file.

#### Config File Structure

The `prog` tool supports nested subcommands and lists of shell commands. These can be written in the form of nested objects and nested lists, respectively. Examples in JSON and YAML are as follows:

```
{
    "config": {
        "debug": "cmake -DCMAKE_BUILD_TYPE=Debug -B build -G Ninja .",
        "release": "cmake -DCMAKE_BUILD_TYPE=Debug -B build -G Ninja ."
    },
    "build": "cd build; ninja",
    "push": [
        "git add .",
        "git commit",
        "git push"
        "git status"
    ]
}
```

```
config:
    debug: cmake -DCMAKE_BUILD_TYPE=Debug -B build -G Ninja .
    release: cmake -DCMAKE_BUILD_TYPE=Release -B build -G Ninja .
build: cd build; ninja
push:
    - git add .
    - git commit
    - git push
    - git status
```

#### Invocation

 Once the JSON file has been configured to your needs, you can simply invoke the
 custom commands using the following command:
 `prog [COMMANDS]...`

 Multiple commands may be specified, and will execute in the order they are specified.

 If nested subcommands are present, the `prog` tool expects exactly one command per layer of nesting. Try enabling verbose output if issues arise. With respect to the above example, a valid command sequence would be written as such:

```
prog config debug build push
```

## Future Plans

 With the addition of advanced object structures to `prog`'s parser, I can see a lot more use cases for this style of program. I plan to update this project incrementally as I get ideas for more intuitive behavior.
