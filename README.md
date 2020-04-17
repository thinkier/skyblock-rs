# skyblock-rs
Rust client for interacting with Hypixel's Skyblock API

## Getting started
`examples/index-ah` is a fairly simple example of how to index into all of the current available auctions, other features will be added as I go along. This library is written using stable rust's futures! So minimum rustc of `1.39` or it won't run. Consult the examples' `Cargo.toml` if in doubt.

Ramblings aside, here's is how to include it in your project:
```toml
[dependencies.skyblock-rs]
git = "https://github.com/thinkier/skyblock-rs"
```

Currently it's highly unstable as it's very much incomplete, but the basic backbone is there, some refactoring in the future will probably cause irritation and yeah.

## Contributions
Yes please! Make pull requests! Just don't borke the whole thing~!
