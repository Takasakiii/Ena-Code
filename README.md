# Ena-Code

A little profile switcher for Visual Studio Code, making it possible to segregate configs and extensions according to the context/lang.

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
$ ecode [PROFILE] [PATH]
```
> Being PROFILE and PATH optional arguments.

### Flags:
- `-b <PROFILE>`:
    The profile used as base to new profiles. (Default: `Default`)
- `-v`
    Enables verbose mode for debugging

Ena Code uses `{userFolder}`/.ena-code folder to save the profiles and settings.

In the configuration file (`.ena-code/config.yml`), it's possible to change the profiles folder, the VSCode executable, the base profile, if the configs are shared between the profiles and if the default folder will be the current folder.

## Ena-Code Manager
Ena-Code manager is an utility that ships with Ena-Code, helping to manage profiles and edit Ena-Code's configuration.

### Usage:

#### Profiles:
- Delete profile:
    ```sh
    $ ecode-manager profiles remove <NAME>
    ```
- List profiles
    ```sh
    $ ecode-manager profiles list
    ```

#### Configuration:
- Change default profile:
    ```sh
    $ ecode-manager config default-profile <PROFILE>
    ```
- Change profiles folder:
    ```sh
    $ ecode-manager config profiles-folder <PATH>
    ```
- Set if the profiles configs are shared:
    ```sh
    $ ecode-manager config shared-profiles-configs <enable/disable>
    ```
- Set if the current folder is the default:
    ```sh
    $ ecode-manager config use-current-folder <enable/disable>
    ```
- Change VSCode path:
    ```sh
    $ ecode-manager config vs-code-path <PATH>
    ```

All subcommands also have a `help` command, showing how you can use it, like:
```sh
$ ecode-manager config help
```

## Ena-Code URL Handler
Ena-Code now has an URL handler, neccessary for handling `vscode://` URLs.

> Note that the URL Handler still doesn't work properly on Windows systems.

### Usage:
```sh
$ ecode-url-handler <URL>
```

This will open a graphical interface that lets you choose the profile to open the URL with:
![URL Handler profile select screen](https://i.imgur.com/wkWPbWY.png)

If you want to open the URLs directly from browser, without using the command line, you can use a custom scheme handler.

### Linux example XDG .desktop file
File based on VSCode's [code-url-handler.desktop](https://github.com/microsoft/vscode/blob/main/resources/linux/code-url-handler.desktop)
```ini
[Desktop Entry]
Name=Ena-Code URL Handler
Comment=URL Handler for Ena-Code
GenericName=Text Editor
Exec=/path/to/ecode-url-handler %U
Type=Application
NoDisplay=true
StartupNotify=true
Categories=Utility;TextEditor;Development;IDE;
MimeType=x-scheme-handler/vscode;
Keywords=vscode;
```

You can put this file in your `~/.local/share/applications`. After running:
```sh
$ update-desktop-database ~/.local/share/applications
```
Ena-Code URL Handler will probably be available while handling `vscode://` URLs.

## License
This project is licensed under [The Unlicense](https://unlicense.org/) license, belonging to the public domain.
