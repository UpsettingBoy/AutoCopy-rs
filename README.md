# AutoCopy
## Aim
The aim of this project is allowing you to _emulate_ the **Steam Cloud** behaviour within your local filesystem.
This can be use to share your local saved-games with another PC (even cross-OS).

## Usage
````
USAGE:
    autocopy-rs.exe [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add       Adds a folder to the copy registry (creates a profile)
    help      Prints this message or the help of the given subcommand(s)
    remove    Removes the selected profile
    show      Shows all added profiles
    start     Starts the program
````
 
## Licenses
This project is licensed under [Apache 2.0](http://www.apache.org/licenses/LICENSE-2.0).
**AutoCopy** makes use of the following 3º party libraries:
+ **[app_dirs2](https://docs.rs/app_dirs2/)**
+ **[clap](https://docs.rs/clap/)**
+ **[copy_dir](https://docs.rs/copy_dir/)**
+ **[diesel](https://docs.rs/diesel/)**
+ **[fs_extra](https://docs.rs/fs_extra/)**
+ **[fs2](https://docs.rs/fs2/)**
+ **[prettytable](https://docs.rs/prettytable/)**
+ **[time](https://docs.rs/time/)**

All of the licenses of these projects can be found on *__third_party__* folder.
