# Language Bindings

This article explores the reasons why the Bitcoin Dev Kit Foundation supports a number of language bindings libraries as part of its core offering, and the challenges and decision tradeoffs we face along the way.

We build language bindings for a number of use cases. One of the most common of those rests on a belief that as time goes on, applications of all kinds will need to interact with the bitcoin protocol. Many of those will be applications that are not "bitcoin-first" like wallets, but rather other kinds of applications that simply wish to integrate payments for their users (games, chat applications, content creation, etc.). These applications already have well-developed codebases and teams, seldom built entirely in Rust (BDK's first and core language). Our goal is to offer these teams and applications an all-inclusive dependency they can add to whatever technology stack they are using in production, and allow the integration of bitcoin-related capabilities without the need to completely change their tech stack or require the hire of full-time bitcoin experts.

Why not simply use libraries that are available in the specific languages? We think the Bitcoin Development Kit is special (of course we do!) for a few reasons:
1. The level of review and number of in-production applications in bitcoin that use the Rust bitcoin ecosystem of libraries is unparalleled (rust-bitcoin, rust-miniscript, and bdk, and so many more!).
2. For the reason above, it is most often the case that new features and BIPs are available in Rust first (taproot, miniscript, etc.) and take years to appear on other tech stacks.

## Awesome! Producing Bindings Must Be Easy Right?

Along the way, _actually_ producing language bindings for a variety of languages is no easy feat. Here are some of the challenges we face:

1. We create bindings for many languages in one fell swoop with a Rust tool called [uniffi](https://github.com/mozilla/uniffi-rs). The result is that for the work of 1 language, we actually get many: Swift, Kotlin (works for JVM server-side and on Android), Java, Python, Flutter (Dart), and React Native (JS).
2. The goal of the bindings is not to provide all the complexity available in the Rust libraries (this would simply be out-of-scope for us). We basically need to strike a balance and generate a unified API that contains and combines many Rust libraries: rust-bitcoin, rust-miniscript, bdk_wallet, bdk_chain, and the electrum, esplora, and kyoto client libraries. This is required because it is impractical to produce bindings libraries for all of the above individually. The final bindings libraries are centered around the bdk_wallet API, and supporting its most common use cases for mobile clients.
3. Point 2 above has interesting implications: while developers using Rust can simply import any number of those libraries in their projects, we must expose as much (and as little) as is required.
4. A few caveats give us interesting puzzles we need to juggle with as we design and develop the language bindings libraries:
	- We cannot expose Rust generics using uniffi. This means that in practice, we need to remove all generics from the Rust API (either by not exposing the underlying construct or by exposing all—or the most important—of its variants as concrete types). In this process, some of the complexity and beauty of the Rust language and Rust-based codebases is "erased".
	- Because the Rust code must be exposed in a variety of languages, some of the most Rust-specific syntax and features cannot be expressed in the bindings libraries. Things like functions that return tuples and tuple structs do not have Kotlin or Swift equivalents, and must therefore be wrapped in some way, changing the shape of the Rust API slightly.

## Our Scope and Approach to Bindings

We can't produce and maintain bindings for all Rust crates we get requests for, but we are working to help others build their own bindings by (1) making our architecture composable and reusable, and (2) building strong examples and documentation on how to do it for other crates.

Over the past few years, the Bitcoin Development Kit team has been successful at building and releasing language bindings for our core Rust library, [bdk_wallet](https://github.com/bitcoindevkit/bdk_wallet/). Over this time, we've had many requests to add to the bindings certain features that are not directly in the Rust BDK library. These requests mainly break down into two groups:

1. Features that _are_ part of crates "upstream" of BDK (rust-bitcoin, rust-miniscript)
2. Features that are _not_, but that have Rust crates and would be useful on mobile (payjoin, coinjoin implementations, silent payments, BIP-47)

### Current architecture

The current architecture for the BDK bindings is more or less wrapping the bdk, rust-bitcoin, and rust-miniscript crates and exposing an API that allows users to leverage them similarly to how they would in Rust if they were using it in a Rust project.

While we started with a simplified version of the Rust BDK API, over time users asked for more and more functionality, and exposing some of the underlying rust-bitcoin constructs became important. This makes sense, and indeed users of the Bitcoin Development Kit in Rust have access to all the related APIs by simply importing rust-bitcoin and rust-miniscript, hence our desire to accommodate these use cases as well. However, this is currently done all in one "bindings" library (i.e. if you import `bdk-android` in a project, you'll have access to an API that is mostly bdk-based, but also contains a bit of rust-bitcoin and rust-miniscript).

### Moving forward: building a family of libraries

At the same time, other Rust-based libraries are using the uniffi approach (a good example is [ldk-node](https://github.com/lightningdevkit/ldk-node)) to expose bindings. When developing and using those libraries together, it quickly became clear that much of the work was duplicated; both libraries needed access to underlying rust-bitcoin types, but they both exposed their own versions of it.

The team is looking at extracting the rust-bitcoin part of the BDK bindings library (bdk-ffi) and publishing that library on [crates.io](https://crates.io/) so as to make it available to others who wish to build Rust bindings using uniffi. This is currently blocked upstream but might become available in future releases of uniffi.

### Why can't we just build one big BDK library with _everything_ in it?

1. The short answer to this is that it would simply not be maintainable. If we rely on many underlying Rust crates, we'd need to release patches every time one of the underlying libraries patches a bug. We'd also need to keep them all in sync (what API versions work with what), and we'd be relying on work from teams that may or may not have the capacity to keep their crates up to date.
2. Scope creep. Unless we define a narrow and structured scope for the library, we will forever be handling requests for features that may or may not be feasible to accommodate.
3. Library size. Because one of our primary focus for the bindings is mobile devices, we need to make sure we don't build a library that is too big. This is a more nuanced issue, but it relates to point (2), where too large a scope would eventually produce a library that is potentially not optimal for mobile devices because it attempts to do too much all in one package.

### Are you looking to build Rust bindings yourself?

We got your back! The Bitcoin Development Kit team intends to help others in the Rust bitcoin ecosystem build bindings if they wish to. To that effect, we maintain repositories that should help you get going with bindings in no time:

1. **[Uniffi library template](https://github.com/thunderbiscuit/uniffi-bindings-template)**. This is a repository you can fork and start adding code to produce bindings directly for iOS and Android. Included are our custom-made Gradle plugin and Swift release shell scripts, as well as information about the little build quirks you need to know about for smooth releases.
2. **[Uniffi examples](https://github.com/thunderbiscuit/uniffi-examples)**. This repository provides boiled-down examples of APIs exposed using uniffi, with an [accompanying documentation website](https://thunderbiscuit.github.io/uniffi-examples/). Functions, enums, objects, callbacks, multi-libraries, a lot of information and examples to get you started.

We also have a Discord server dedicated to uniffi-based libraries in bitcoin. Reach out on our standard Discord for an invite!
