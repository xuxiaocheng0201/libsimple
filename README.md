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


# Example

```rust
use anyhow::Result;
use tempfile::tempdir;

fn main() -> Result<()> {
    let dir = tempdir()?;
    libsimple::initialize(&dir)?;
    let conn = rusqlite::Connection::open_in_memory()?;
    libsimple::load(&conn)?;
    conn.execute_batch("
        CREATE TABLE singer (id INTEGER, name TEXT);
        CREATE VIRTUAL TABLE d USING fts5(id, name, tokenize = 'simple');
        CREATE TRIGGER insert_trigger AFTER INSERT ON singer BEGIN
            INSERT INTO d(id, name) VALUES (new.id, new.name);
        END;
        INSERT INTO singer (id, name) VALUES (1, '周杰伦');
    ")?;
    assert_eq!(conn.query_row("SELECT id FROM d WHERE name MATCH simple_query('zhoujiel')",
                              [], |row| row.get::<_, i64>(0))?, 1);
    Ok(())
}
```


# Note

To build this crate,
please make sure you have installed `make` and `cmake`.

This crate will clone the [cppjieba](https://github.com/yanyiwu/cppjieba.git/) from GitHub when you build,
so make sure you have the network access.


# License

Licensed under MIT license ([LICENSE-MIT](LICENSE) or http://opensource.org/licenses/MIT)
