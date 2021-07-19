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

### Arch Linux

You can install venv-wrapper from the AUR.

```bash
$ paru -S venv-wrapper-bin
```

### Homebrew

```bash
$ brew install marier-nico/packages/venv-wrapper
```

### PPA

Coming soon!

## Getting Started

1. After installing, make sure you can use the `venv-wrapper` command (it should be in your shell's
`$PATH`).
2. Then, you need to setup your shell to use venv-wrapper. To do this, edit your shell
startup configuration file (`~/.bashrc`, `~/.zshrc`, `~/.config/fish/config.fish`) and add the
following anywhere in there :

### Bash

```bash
eval "$(venv-wrapper init bash)"
venv completions # Optional, if you want shell completions
```

### ZSH

```zsh
eval "$(venv-wrapper init zsh)"
venv completions # Optional, if you want shell completions
```

### Fish

```bash
venv-wrapper init fish | source
venv completions # Optional, if you want shell completions
```

## Configuration

It's possible to configure venv-wrapper with either a configuration file, environment variables, or
CLI flags.

### Available Configuration Values

- `venv_root`: The directory in which to store all virtualenvs (defaults to `~/.virtualenvs`).

### Config File

#### Config Content

The config file is a simple `ini` file that contains no sections, like this :

```ini
venv_root = /home/me/.non-default-location
```

**CAUTION**: Paths in your configuration MUST be absolute, otherwise you might end up putting
virtual environments where you don't intend to.

#### Config Location

The location for the configuration file depends on your platform of choice. For specific
implementation details, see the [directories](https://docs.rs/directories/3.0.2/directories/) crate.

- **Linux**

    The [XDG user directory](https://www.freedesktop.org/wiki/Software/xdg-user-dirs/) specification is
    followed. Assuming defaults, the configuration should be located in
    `~/.config/venv-wrapper/config.ini`.

- **macOS**

    The [Standard Directories](https://developer.apple.com/library/content/documentation/FileManagement/Conceptual/FileSystemProgrammingGuide/FileSystemOverview/FileSystemOverview.html#//apple_ref/doc/uid/TP40010672-CH2-SW6)
    are used. By default, the configuration should be in `~/Library/Application support/venv-wrapper/config.ini`

### Environment Variables

Note that paths do not need to be absolute here because your shell will perform path expansion.
You _do_ need the path to be absolute if your shell does not expand paths.

```bash
VENVWRAPPER_VENV_ROOT=~/.a-different-venvs-directory venv ls
```

### CLI Flags

The same note as with environment variables applies here : no need for an absolute path unless your
shell does not expand paths.

- `venv -r ~/.a-different-venvs-directory venv ls`

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
6. After the release is created, update the homebrew formula [here](https://github.com/marier-nico/homebrew-packages/blob/main/Formula/venv-wrapper.rb).
