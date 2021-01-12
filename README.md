<h2 align="center">Simple Python Virtual Environment Management</h2>

venv-wrapper is a convenient wrapper around Python's `venv` module. It was inspired by the
[virtualenvwrapper](https://bitbucket.org/virtualenvwrapper/virtualenvwrapper/src/master/) project,
and it aims to be simpler and more convenient to use.

## Installation

Coming soon, to [crates.io](https://crates.io/), the AUR, a PPA and homebrew!

## Configuration

It's possible to configure venv-wrapper with either a configuration file, environment variables, or
CLI flags.

- `~/.config/venv-wrapper/config.toml`
    - All paths in the config file must be absolute.
    ```toml
    venvs_dir = "/home/username/.a-different-venvs-directory"
    ```

- `VENVWRAPPER_VENVS_DIR=~/.a-different-venvs-directory venv ls`
    - Paths do not need to be absolute with environment variables.

- `venv -d ~/.a-different-venvs-directory venv ls`
    - Paths do not need to be absolute with CLI flags.

### Available Configuration Values

- `venvs_dir`: The directory in which to store all virtualenvs.

## Shell Compatibility

Currently, only `sh`, `bash` and `zsh` have been tested and confirmed to work. However, `fish`
support should be comming soon!

## UTF-8

Any path to a virtualenv (including the virtualenv name itself) must be valid UTF-8 because paths
and virtualenv names are printed to the terminal (and writing invalid UTF-8 to the terminal wouldn't
really be all that useful).

Though, since emojis are valid UTF-8, feel free to use them in your virtualenv names! 🚀

## Missing Features

In its current state, this project does not quite match the features of `virtualenvwrapper`. The
missing features are as follows :

- Copy virtualenvs
- Tab completion of virtualenv names
- Customizable hooks
- A plugin system to create shareable extensions

## Releasing Versions

To release a new version, there a few simple steps to follow.

1. Create or edit the `RELEASE_CHANGELOG.md` file (at the repo's root) to contain a changelog for the release.
    - This will be the GitHub release's body
2. Merge all code to be released into `main`.
3. Create a new tag pointing to the head of the `main` branch.
    - `git tag -s vX.Y.Z -m "Release vX.Y.Z"`
4. Push the new tag.
    - `git push --tags`
