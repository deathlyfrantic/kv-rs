# kv.rs

A basic CLI key:value store. A Rust rewrite of
[kv](https://github.com/deathlyfrantic/kv).

## Installation

`cargo install --path .`

## Usage

    USAGE:
        kv [SUBCOMMAND]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    SUBCOMMANDS:
        delete    Deletes key:value pairs.
        get       Gets the value for a given key.
        help      Prints this message or the help of the given subcommand(s)
        list      Lists all key:value pairs.
        set       Sets a value for a key.

## zsh Completion

A zsh script is included for optional completion. Ensure the `_kv` file is in
your `$fpath` and this should work automatically.

## License

BSD 2-clause
