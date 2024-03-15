# connect-1password

[![crates.io](https://img.shields.io/crates/v/connect-1password?color=orange&label=crates.io)](https://crates.io/crates/connect-1password)
[![Released API docs](https://img.shields.io/docsrs/connect-1password)](https://docs.rs/connect-1password/)

The 1Password Connect Rust SDK provides your Rust applications access to the 1Password Connect API hosted on your infrastructure and leverage the power of 1Password Secrets Automation

The library can be used by Rust applications, tools, and other automations to access and manage items in 1Password Vaults.

## Installation

## Usage

1. Start by copying `.env-sample` to `.env`, making sure to update its values.
2. Follow the [instructions to start Connect](https://github.com/1Password/connect) and make sure the Docker container starts at `http://localhost:8080`
3. Make sure to update `OP_API_TOKEN` in the `.env` file.


## API usage

Refer to the [docs](https://docs.rs/connect-1password/0.1.0/connect_1password/) for further examples.

### Upcoming enhancements

- [ ] The current goal is to cover all existing API end-points (in progress).
- [ ] Setup Github CI with account donated by 1Password (thanks team!)
- TBD: If you have any requests, please open an issue!

## Development

### Running Tests

From repository root:

```shell script
cargo test
```

### Building

```shell script
cargo build --release
```

## This is not an official SDK

This Rust crate has been created solely by me and so far is not "officially" supported by the
awesome folk at 1Password.  This may change, maybe and if they would like to help out and/or adopt
this into the 1Password ecosystem, that would be nice as well.

---

# About 1Password

**[1Password](https://1password.com/)** is the world’s most-loved password manager. By combining industry-leading security and award-winning design, the company provides private, secure, and user-friendly password management to businesses and consumers globally. More than 60,000 business customers trust 1Password as their enterprise password manager.

# Security

1Password requests you practice responsible disclosure if you discover a vulnerability. Please submit discoveries via [BugCrowd](https://bugcrowd.com/agilebits).

For information about security practices, please visit our [Security homepage](https://1password.com/security/).

# MSRV

This project is tested against the most [recent stable rust version](https://gist.github.com/alexheretic/d1e98d8433b602e57f5d0a9637927e0c).

# License

This work is licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
  <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.