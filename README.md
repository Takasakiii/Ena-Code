# Ena-Code

A little profile switcher for Visual Studio Code, making it possible to segregate configs and extensions according to the context/lang.

**This project is still in alpha and may have many bugs and unfinished things, but your main function is usable.**

## Bulding and installation:

### Dependencies:
- [Rust](https://www.rust-lang.org/)

### Steps:
Install the latest version from [crates.io](https://crates.io/crates/ecode):
```sh
$ cargo install ecode
```

Or, in the project folder, use cargo to install from the source:
```sh
$ cargo install --path .
```

## Usage:
```sh
$ ecode [profile] [path]
```
> Being [profile] and [path] optional arguments.

Ena Code uses `{userFolder}`/.ena-code folder to save the profiles and settings.

In the configuration file (`.ena-code/config.yml`), it's possible to change the profiles folder, the VSCode executable, the base profile, if the configs are shared between the profiles and if the default folder will be the current folder.

## Ena-Code Manager
Ena-Code manager is an utility that ships with Ena-Code, helping to manage profiles.

### Usage:

- Delete profile:
    ```sh
    $ ecode-manager profiles remove <NAME>
    ```
- List profiles
    ```sh
    $ ecode-manager profiles list
    ```

## License
This project is licensed under [The Unlicense](https://unlicense.org/) license, belonging to the public domain.
