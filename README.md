<h2 align="center">Simple Python Virtual Environment Management</h2>

venv-wrapper is a convenient tool to help you manage your Python virtual environments. It used to be
a wrapper around python's built-in `venv` module, but that is no longer the case, the required
features from `venv` were re-implemented in Rust instead.

## Installation

### Cargo and crates.io

1. Install rust (https://www.rust-lang.org/tools/install).
2. Add the `cargo` bin path to your `PATH` environment variable.
    - Usually the bin path is `~/.cargo/bin`.
3. Run `cargo install venv-wrapper`.

-----------------------------------------------------------------
4. Add `eval "$(venv-wrapper init)"` to your shell init script (`~/.bashrc`, `~/.zshrc`, etc.)
5. Restart your shell.
6. You can now run `venv ls` to verify the installation is working.
-----------------------------------------------------------------

### Arch Linux

You can install venv-wrapper from the AUR.

```bash
$ paru -S venv-wrapper-bin
```

### PPA, Homebrew

Coming soon!

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

- `venv_root`: The directory in which to store all virtualenvs (defaults to `~/.virtualenvs`).

## Shell Compatibility

These shells are fully supported and _should_ all work correctly :
- Bash
- ZSH
- Fish

However, all features except shell completions should work in most bash-like shells.

## Shell Completions

By default, shell completions are not active, but enabling them is really easy. All you have to do
is run :

```bash
$ venv completions
```

**Note:** You must initialize venv-wrapper in your shell before activating completions.

# Contributing

## Getting Started

Mostly, you should install [pre-commit](https://pre-commit.com/) and run `pre-commit install` to
make sure your commits are up to stuff! Also, your commits should adhere to
[conventional commits](https://www.conventionalcommits.org/en/v1.0.0/). To do this, you can use a
tool like [commitizen](https://github.com/commitizen-tools/commitizen), which will help make sure
all commits look good.

The commit convention was added recently, so most commits are not yet compliant!

## Missing Features

In its current state, this project does not quite match the features of `virtualenvwrapper`. The
missing features are as follows :

- Copy virtualenvs
- Customizable hooks
- A plugin system to create shareable extensions

## Releasing Versions

To release a new version, there a few simple steps to follow.

1. Create or edit the `RELEASE_CHANGELOG.md` file (at the repo's root) to contain a changelog for the release.
    - This will be the GitHub release's body
2. Update the version in `cargo.toml`, and `src/cli/get_app.rs`.
3. Merge all code to be released into `main`.
4. Create a new tag pointing to the head of the `main` branch.
    - `git tag -s vX.Y.Z -m "Release vX.Y.Z"`
5. Push the new tag.
    - `git push --tags`
