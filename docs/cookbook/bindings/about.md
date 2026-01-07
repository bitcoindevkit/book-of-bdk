# About

The BDK libraries are also available in a multitude of other programming languages. These language bindings libraries all share a common API, which is defined in the [bdk-ffi](https://github.com/bitcoindevkit/bdk-ffi) repository. You can read more about [why we do this here](https://bitcoindevkit.org/blog/why-bindings/) and [how we do it here](https://bitcoindevkit.org/blog/bindings-scope/).

The "common API" is in fact a wrapper around [9 different Rust libraries](https://bitcoindevkit.org/blog/why-bindings/#awesome-producing-bindings-must-be-easy-right), allowing us to produce a single artifact users of language bindings can import as a single dependency.

The common API is useful for a few reasons:

1. While each language has its own syntax, all users of bdk-ffi language bindings share the same API. This ensures we have users in production in many contexts, and feature requests and bug reports are all handled in one place.
2. Documentation and examples of this API are useful for all languages.
3. PRs and community contributions that help one language help all languages.
 