# Contributing to Artichoke – Boba

👋 Hi and welcome to [Artichoke]. Thanks for taking the time to contribute!
💪💎🙌

Artichoke aspires to be a Ruby 2.6.3-compatible implementation of the Ruby
programming language. [There is lots to do].

Boba is used to implement an obscure function in the [`Digest` package].

If Artichoke does not run Ruby source code in the same way that MRI does, it is
a bug and we would appreciate if you [filed an issue so we can fix it]. [File
bugs specific to Boba in this repository].

If you would like to contribute code to Boba 👩‍💻👨‍💻, find an issue that looks
interesting and leave a comment that you're beginning to investigate. If there
is no issue, please file one before beginning to work on a PR. [Good first
issues are labeled `E-easy`].

## Discussion

If you'd like to engage in a discussion outside of GitHub, you can [join
Artichoke's public Discord server].

## Setup

Boba includes Rust, Ruby, and Text sources. Developing on Boba requires
configuring several dependencies.

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

### Ruby

Boba requires a recent Ruby 2.x and [bundler] 2.x. The
[`.ruby-version`](.ruby-version) file in this repository specifies Ruby 2.6.6.

Boba uses [`rake`](Rakefile) as a task runner. You can see the available tasks
by running:

```console
$ bundle exec rake --tasks
rake build                        # Build Rust workspace
rake bundle:audit:check           # Checks the Gemfile.lock for insecure dependencies
rake bundle:audit:update          # Updates the bundler-audit vulnerability database
rake doc                          # Generate Rust API documentation
rake doc:open                     # Generate Rust API documentation and open it in a web browser
rake fmt                          # Format sources
rake fmt:rust                     # Format Rust sources with rustfmt
rake fmt:text                     # Format text, YAML, and Markdown sources with prettier
rake format                       # Format sources
rake format:rust                  # Format Rust sources with rustfmt
rake format:text                  # Format text, YAML, and Markdown sources with prettier
rake lint                         # Lint sources
rake lint:clippy                  # Lint Rust sources with Clippy
rake lint:clippy:restriction      # Lint Rust sources with Clippy restriction pass (unenforced lints)
rake lint:rubocop                 # Run RuboCop
rake lint:rubocop:auto_correct    # Auto-correct RuboCop offenses
rake release:markdown_link_check  # Check for broken links in markdown files
rake test                         # Run Boba unit tests
```

To lint Ruby sources, Boba uses [RuboCop]. RuboCop runs as part of the `lint`
task. To run RuboCop by itself, invoke the `lint:rubocop` task.

```console
$ bundle exec rake lint
$ bundle exec rake lint:rubocop
```

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

To lint and format all sources run:

```sh
rake lint
```

## Testing

A PR must have new or existing tests for it to be merged. The [Rust book chapter
on testing] is a good place to start.

To run tests:

```sh
cargo test
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
  https://ruby-doc.org/stdlib-2.6.3/libdoc/digest/rdoc/Digest.html#method-c-bubblebabble
[filed an issue so we can fix it]:
  https://github.com/artichoke/artichoke/issues/new
[file bugs specific to boba in this repository]:
  https://github.com/artichoke/boba/issues/new
[good first issues are labeled `e-easy`]:
  https://github.com/artichoke/boba/labels/E-easy
[join artichoke's public discord server]: https://discord.gg/QCe2tp2
[rustup]: https://rustup.rs/
[homebrew]: https://docs.brew.sh/Installation
[bundler]: https://bundler.io/
[rubocop]: https://github.com/rubocop-hq/rubocop
[prettier]: https://prettier.io/
[node.js]: https://nodejs.org/en/download/package-manager/
[rust book chapter on testing]:
  https://doc.rust-lang.org/book/ch11-00-testing.html
[cargo-outdated]: https://github.com/kbknapp/cargo-outdated
[artichoke/artichoke#548]: https://github.com/artichoke/artichoke/pull/548
[@dependabot]: https://dependabot.com/
