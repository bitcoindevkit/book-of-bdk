# Book of BDK

This repository hosts the code and content for the [Book of BDK website](https://bitcoindevkit.github.io/book-of-bdk/).
We use [`mkdocs-material`](https://squidfunk.github.io/mkdocs-material) to render the website and the content.

## Develop locally

To develop locally, you'll need to install `mkdocs-material`:

```shell
pip install mkdocs-material
```

You can serve the site with [`just`](https://just.systems/man/en/):

```shell
just serve
```

All the rust code can be tested with:

```shell
just test
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
