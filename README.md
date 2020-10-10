# node-cleaner ![build](https://github.com/KaranGauswami/node-cleaner/workflows/Rust/badge.svg) ![release](https://img.shields.io/github/v/release/KaranGauswami/node-cleaner?include_prereleases)

Simple script to remove all the node modules from older projects

## About

`node-cleaner` purpose is to clean old node_modules from the old projects.


## How it works

The functionality of `node-cleaner` is to check in the specified directory in option if the last modified time of the directory is older than 30 days then it will remove node_modules folder from that directory.


## Compiling

Follow these instructions to compile `node-cleaner`, then skip down to Installation.

 1. Ensure you have current version of `cargo` and [Rust](https://www.rust-lang.org) installed
 2. Clone the project `$ https://github.com/KaranGauswami/node-cleaner && cd node-cleaner`
 3. Build the project `$ cargo build --release`
 4. Once complete, the binary will be located at `target/release/node-cleaner`


## Usage


### Options

There are a few options for using `node-cleaner`.

```text
node-cleaner 0.1.0

USAGE:
    node-modules-cleaner [FLAGS] [OPTIONS]

FLAGS:
    -f, --force      whether or not move files to trash instead of permenant deletion
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --days <days>        Duration in days [default: 30]
    -p, --path <path>        Path to run the script [default: .]
    -t, --target <target>    Directory to remove [default: node_modules]
```


## License

`node-cleaner` is released under the terms of either the MIT or Apache 2.0 license. See the LICENSE-MIT or LICENSE-APACHE file for the details.

