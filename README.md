# sdmx
[![License][license-badge]][license-url]
[![CI][ci-badge]][ci-url]
[![Security audit][security-badge]][security-url]

[license-badge]: https://img.shields.io/badge/License-MIT%20%26%20Apache%202.0-blue?style=flat-square
[license-url]: #license
[ci-badge]: https://img.shields.io/github/deployments/neoncitylights/sdmx/github-pages?label=deploy&style=flat-square
[ci-url]: https://github.com/neoncitylights/sdmx/actions/workflows/main.yml
[security-badge]: https://img.shields.io/github/actions/workflow/status/neoncitylights/sdmx/.github/workflows/main.yml?style=flat-square
[security-url]: https://github.com/neoncitylights/sdmx/actions/workflows/security-audit.yml

This monorepo provides Rust-related crates to SDMX (Statistical Data and Metadata eXchange). At the moment, this currently implements [SDMX-JSON](https://github.com/sdmx-twg/sdmx-json) and [SDMX-CSV](https://github.com/sdmx-twg/sdmx-csv), and may provide a crate for [SDMX-ML](https://github.com/sdmx-twg/sdmx-ml) in the future.

| Crate     | Status | crates.io | docs.rs |
| --------- | ------ | --------- | ------- |
| [`sdmx_json`](./crates/sdmx_json) | Beta stage | [crates.io](https://crates.io/crates/sdmx_json) | [docs.rs](https://docs.rs/sdmx_json) |
| [`sdmx_csv`](./crates/sdmx_csv) | Alpha stage | *N/A* | *N/A* |

## License
Licensed under either of

- Apache License, Version 2.0 ([`LICENSE-APACHE`](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([`LICENSE-MIT`](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

### Contribution
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
