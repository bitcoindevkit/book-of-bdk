# Book of BDK

This repository hosts the code and content for the [Book of BDK website](https://bitcoindevkit.github.io/book-of-bdk/).
We use [`mkdocs-material`](https://squidfunk.github.io/mkdocs-material) to render the website and the content.

## Develop locally

To develop locally, you'll need to install `mkdocs-material`:

First you'll likely want to open a python virtual environment (since python environment management can be truly nightmarish otherwise). Doing so will depend on your OS.

Here's an example using [venv](https://docs.python.org/3/library/venv.html) on MacOS. Set up a virtual environment if you don't already have one:
```shell
mkdir ~/.venv
python3 -m venv ~/.venv
```
Then start the virtual environment:
```shell
source ~/.venv/bin/activate
```

Then you're ready to install:

```shell
pip install mkdocs-material
```

You can serve the site with [`just`](https://just.systems/man/en/):

```shell
just serve
```

All the rust code can be compiled with:

```shell
just rust
```

NOTE: Some tests need additional dependencies on macOS such as `libiconv`.

A [Nix](https://nixos.org) shell can be instantiated with:

```nix
with import <nixpkgs> { };
mkShell {
  nativeBuildInputs = [
    bashInteractive
    python311Packages.mkdocs-material
    # macOS specifics
    libiconv
    darwin.apple_sdk.frameworks.Security
    darwin.apple_sdk.frameworks.SystemConfiguration
  ];
}
```
