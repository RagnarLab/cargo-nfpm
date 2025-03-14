<div align="center">
  <h1>cargo-nfpm</h1>
  <p>
    <strong>A simple and lightweight Cargo plugin for using <a href="https://nfpm.goreleaser.com/">nFPM</a> from any Rust project.</strong>
  </p>
  <p>
</div>

[![Build Status](https://github.com/ragnarlab/cargo-nfpm/actions/workflows/ci.yml/badge.svg)](https://github.com/ragnarlab/cargo-nfpm/actions)
[![Crates.io](https://img.shields.io/crates/v/cargo-nfpm.svg)](https://crates.io/crates/cargo-nfpm)
![License](https://img.shields.io/crates/l/cargo-nfpm.svg)

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
      --no-build
          Whether to skip the build
      --no-vendor
          Whether to skip downloading nFPM
  -f, --format <FORMAT>
          Output format [possible values: apk, arch-linux, deb, ipk, rpm]
  -h, --help
          Print help
  -V, --version
          Print version
```

## Support

> Development of cargo-nfpm is sponsored by [RagnarLab](https://ragnarlab.com). RagnarLab is a Rust consultancy based in Stuttgart, Germany. We provide Rust development from prototype to product, helping you write safer software. [Interested in Rust? Get in touch with us.](https://ragnarlab.com)

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

[nFPM]: https://nfpm.goreleaser.com/
