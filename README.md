# `cfg_feature_groups`

> define feature groups to improve conditional compilation management

[![Build Status](https://travis-ci.org/little-arhat/cfg_feature_groups.svg?branch=master)](https://travis-ci.org/little-arhat/cfg_feature_groups)
[![crates.io](http://meritbadge.herokuapp.com/cfg_feature_groups?style=flat-square)](https://crates.io/crates/cfg_feature_groups)

This library allows one to define feature group, that may only take one
value out of set. Feature groups defined in `Cargo.toml` as metadata:

```Cargo.toml
[package.metadata.feature_groups]
log = ["dummy", "semihosting", "itm"]
```

Where `"dummy", "semihosting", "itm"` are features defined separately.
Once feature groups are defined, they can be checked during build time
and turned into cfg attributes:

```Cargo.toml
[build-dependencies]
cfg_feature_groups = "..."
```

```build.rs
use cfg_feature_groups::setup_feature_groups;
fn main() {
    setup_feature_groups();
}
```

Then in your program you may use something like this:
```main.rs
#[cfg(log = "itm")]
fn define_itm() {}
```

`setup_feature_groups` will ensure that one and only one option is defined
for each feature group.

See [full example](./example).

## Documentation

API Docs available on [docs.rs](https://docs.rs/cfg_feature_groups).

## License

- MIT license ([LICENSE](LICENSE) or http://opensource.org/licenses/MIT)

## See also

[if_cfg](https://github.com/alexcrichton/cfg-if) crates provides alternative way to tackle conditional compilation.
