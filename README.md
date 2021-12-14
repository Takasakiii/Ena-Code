# Ena Code

A little profile switcher for Visual Studio Code, making it possible to segregate configs and extensions according to context/lang.

**This project is still in alpha and may have many bugs and unfinished things, but your main function is usable.**

## Bulding and installation:

### Dependencies:
- [Rust](https://www.rust-lang.org/)

### Steps:
In the project folder, use cargo to install the project:
```sh
$ cargo install --path .
```

Or install the latest version from [crates.io](https://crates.io/crates/ecode):
```sh
$ cargo install ecode
```

## Use:
```sh
$ ecode [profile] [path]
```
> Being [profile] and [path] optional arguments.

Ena Code uses `{userFolder}`/.ena-code to save the profiles and settings.

In the configuration file (`.ena-code/config.yml`), it's possible to change the profiles folder, the VSCode executable, the base profile, if the configs are shared between the profiles and if the default folder will be the current folder.
