# libsimple

[![Crate](https://img.shields.io/crates/v/libsimple.svg)](https://crates.io/crates/libsimple)
[![GitHub last commit](https://img.shields.io/github/last-commit/xuxiaocheng0201/libsimple)](https://github.com/xuxiaocheng0201/libsimple/commits/master)
[![GitHub issues](https://img.shields.io/github/issues-raw/xuxiaocheng0201/libsimple)](https://github.com/xuxiaocheng0201/libsimple/issues)
[![GitHub pull requests](https://img.shields.io/github/issues-pr/xuxiaocheng0201/libsimple)](https://github.com/xuxiaocheng0201/libsimple/pulls)
[![GitHub](https://img.shields.io/github/license/xuxiaocheng0201/libsimple)](https://github.com/xuxiaocheng0201/libsimple/blob/master/LICENSE)

# Description

Rust bindings to [simple](https://github.com/wangfenjin/simple),
a SQLite3 fts5 tokenizer which supports Chinese and PinYin.


# Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
libsimple = "~0.1"
```


# Note

This crate will clone the [cppjieba](https://github.com/yanyiwu/cppjieba.git/) from github when you build,
so make sure you have the network access.


# License

Licensed under MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
