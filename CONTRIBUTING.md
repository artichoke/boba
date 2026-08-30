# Contributing to Artichoke – Boba

👋 Hi and welcome to [Artichoke]. Thanks for taking the time to contribute!
💪💎🙌

Artichoke aspires to be a [recent MRI Ruby][mri-target]-compatible
implementation of the Ruby programming language. [There is lots to do].

[mri-target]:
  https://github.com/artichoke/artichoke/blob/trunk/RUBYSPEC.md#mri-target

Boba is used to implement an obscure function in the [`Digest` package].

If Artichoke does not run Ruby source code in the same way that MRI does, it is
a bug and we would appreciate if you [filed an issue so we can fix it]. [File
bugs specific to Boba in this repository].

If you would like to contribute code to Boba 👩‍💻👨‍💻, find an issue that looks
interesting and leave a comment that you're beginning to investigate. If there
is no issue, please file one before beginning to work on a PR. [Good first
issues are labeled `E-easy`].

## Setup

Boba includes Rust and text sources. Developing on Boba requires configuring
several dependencies.

### Rust Toolchain

Boba depends on Rust and several compiler plugins for linting and formatting.
Boba is guaranteed to build on the latest stable release of the Rust compiler.

#### Installation

The recommended way to install the Rust toolchain is with [rustup]. On macOS,
you can install rustup with [Homebrew]:

```sh
brew install rustup-init
rustup-init
```

Once you have rustup, you can install the Rust toolchain needed to compile Boba:

```sh
rustup toolchain install stable
rustup component add rustfmt
rustup component add clippy
```

To update your stable Rust compiler to the latest version, run:

```sh
rustup update stable
```

### Rust Crates

Boba depends on several Rust libraries, or crates. Once you have the Rust
toolchain installed, you can install the crates specified in
[`Cargo.toml`](Cargo.toml) by running:

```sh
cargo build
```

### Development tasks

Boba uses [mise] to manage its development toolchain and tasks. Install the
toolchain and text-formatting dependencies with:

```sh
mise install
mise run pnpm-install
```

Run `mise tasks` to list the available build, test, lint, format, and
documentation tasks.

### Node.js

Node.js is an optional dependency that is used for formatting text sources with
[prettier].

Node.js is only required for formatting if modifying the following filetypes:

- `md`
- `yaml`
- `yml`

You will need to install [Node.js].

On macOS, you can install Node.js with [Homebrew]:

```sh
brew install node
```

## Linting

To lint all sources run:

```sh
mise run lint
```

## Testing

A PR must have new or existing tests for it to be merged. The [Rust book chapter
on testing] is a good place to start.

To run tests:

```sh
mise run test
```

`cargo test` accepts a filter argument that will limit test execution to tests
that substring match. For example, to run all of the tests for the encoder:

```sh
cargo test encode
```

Tests are run for every PR. All builds must pass before merging a PR.

## Updating Dependencies

### Rust Crates

Version specifiers in `Cargo.toml` are NPM caret-style by default. A version
specifier of `4.1.2` means `4.1.2 <= version < 5.0.0`.

To see what crates are outdated, you can use [cargo-outdated].

If you need to pull in an updated version of a crate for a bugfix or a new
feature, update the version number in `Cargo.toml`. See
[artichoke/artichoke#548] for an example.

Regular dependency bumps are handled by [@dependabot].

[artichoke]: https://github.com/artichoke
[there is lots to do]: https://github.com/artichoke/artichoke/issues
[`digest` package]:
  https://ruby-doc.org/stdlib-3.1.2/libdoc/digest/rdoc/Digest.html#method-c-bubblebabble
[filed an issue so we can fix it]:
  https://github.com/artichoke/artichoke/issues/new
[file bugs specific to boba in this repository]:
  https://github.com/artichoke/boba/issues/new
[good first issues are labeled `e-easy`]:
  https://github.com/artichoke/boba/labels/E-easy
[rustup]: https://rustup.rs/
[homebrew]: https://docs.brew.sh/Installation
[mise]: https://mise.jdx.dev/
[rubocop]: https://github.com/rubocop-hq/rubocop
[prettier]: https://prettier.io/
[node.js]: https://nodejs.org/en/download/package-manager/
[rust book chapter on testing]:
  https://doc.rust-lang.org/book/ch11-00-testing.html
[cargo-outdated]: https://github.com/kbknapp/cargo-outdated
[artichoke/artichoke#548]: https://github.com/artichoke/artichoke/pull/548
[@dependabot]: https://dependabot.com/
