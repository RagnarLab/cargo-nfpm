<div align="center">
  <h1>cargo-nfpm</h1>
  <p>
    <strong>A simple and lightweight Cargo plugin for using <a href="https://nfpm.goreleaser.com/">nFPM</a> from any Rust project.</strong>
  </p>
  <p>

[![Build Status](https://github.com/ragnarlab/cargo-nfpm/actions/workflows/ci.yml/badge.svg)](https://github.com/ragnarlab/cargo-nfpm/actions)
[![Crates.io](https://img.shields.io/crates/v/cargo-nfpm.svg)](https://crates.io/crates/cargo-nfpm)
![License](https://img.shields.io/crates/l/cargo-nfpm.svg)

</div>


## Installation

To install the latest stable release:

```
cargo install --locked cargo-nfpm
```

Or install the latest development version:

```
cargo install --locked --git https://github.com/RagnarLab/cargo-nfpm
```


## Usage

To create a `.deb`/`.rpm`/`.apk` or `.ipk` from a Cargo Rust project requires
no additional setup.

```
$ cargo nfpm package -f deb
using deb packager...
created package: /home/ubuntu/example/target/tmp/example_1.4.2-1_amd64.deb
```

## Config

Instead of configuring `nfpm` via it's own yaml configuration (`nfpm.yml`) the
entire configuration is contained in the extra metadata in `Cargo.toml`.

The following configuration (from `test-projects/single-project`) ...

```toml
# Cargo.toml
[package]
name = "single-project"
version = "0.1.0"
authors = ["Arvid Gerstmann <github@arvid.io>"]
edition = "2024"

[package.metadata.nfpm]
epoch = "2"
contents = [
    { src = "LICENSE", dst = "/usr/share/licenses/single-project/LICENSE" },
]
```

... will be automatically translated into:

```yaml
# nfpm.yml

name: single-project
arch: amd64
version: 0.1.0
contents:
- dst: /usr/share/licenses/single-project/LICENSE
  src: LICENSE
- dst: /usr/bin/single-project
  expand: false
  file_info:
    mode: 493
  src: /home/ubuntu/cargo-nfpm/test-projects/single-project/target/release/single-project
epoch: '2'
maintainer: Arvid Gerstmann <github@arvid.io>
platform: linux
priority: extra
release: '1'
section: default
```

With the necessary fields already filled out with the general metdata contained
in the `package` section of your `Cargo.toml`.

The types are generated from the latest `nfpm` JSON schema and will always
track the latest version.

All possible options are demonstrated in [all.toml](./fixtures/all.toml) which
mirrors the [reference configuration from
nfpm](https://nfpm.goreleaser.com/configuration/).


## Full Options

```
Usage: cargo nfpm package [OPTIONS] --format <FORMAT> [BUILD_OPTIONS]...

Arguments:
  [BUILD_OPTIONS]...
          Build options passed to `cargo build`

Options:
  -p, --package <SPEC>
          Package to build (see `cargo help pkgid`)

      --target <TRIPLE>
          Build for the target triple

      --profile <PROFILE-NAME>
          Build artifacts with the specified profile

  -F, --features <FEATURES>
          List of features to activate

  -o, --output <PATH>
          Where to save the generated package

      --no-build
          Whether to skip the build

      --no-vendor
          Whether to skip downloading nFPM

  -f, --format <FORMAT>
          Package format

          [possible values: apk, archlinux, deb, ipk, rpm]

  -s, --strip <STRIP>
          Strip action

          [default: skip]

          Possible values:
          - skip:             Don't do anything. Default
          - keep-line-tables: Keep only line-tables debug information. This is the minimum requirement for stacktraces
          - strip:            Strip off all debug information. Information is lost if not preserved otherwise
          - split:            Debug information is saved in a separate .debug file

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```


## Environment Variables

cargo-nfpm reads some of the options also as environment variables. Passing the
command-line flag overrides the values from the environment.

|       Variable       |                        Description                        |
| -------------------- | --------------------------------------------------------- |
| `CARGO_NFPM_BIN`     | Path to the `nfpm` binary if `--no-vendor` is passed      |

The following environment variables to emulate Cargo's behavior.

|       Variable       |                        Description                        |
| -------------------- | --------------------------------------------------------- |
| `CARGO`              | Path to the `cargo` binary                                |
| `CARGO_TERM_COLOR`   | Default color mode: `always`, `auto`, or `never`          |
| `CARGO_TARGET_DIR`   | Where to place generated artifacts                        |


## Support

> Development of cargo-nfpm is sponsored by [RagnarLab](https://ragnarlab.com). RagnarLab is a Rust consultancy based in Stuttgart, Germany. We provide Rust development from prototype to product, helping you write safer software. [Interested in Rust? Get in touch with us.](https://ragnarlab.com)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

[nFPM]: https://nfpm.goreleaser.com/
