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
libsimple = "~0.3"
```


# Example

```rust
use anyhow::Result;
use tempfile::tempdir;

fn main() -> Result<()> {
    libsimple::enable_auto_extension()?;
    let dir = tempdir()?;
    libsimple::release_dict(&dir)?;
    
    let conn = rusqlite::Connection::open_in_memory()?;
    libsimple::set_dict(&conn, &dir)?;
    
    conn.execute_batch("
        CREATE VIRTUAL TABLE d USING fts5(id, text, tokenize = 'simple');
        INSERT INTO d (id, text) VALUES (1, '中华人民共和国国歌');
        INSERT INTO d (id, text) VALUES (2, '周杰伦');
    ")?;
    assert_eq!(1, conn.query_row(
        "SELECT id FROM d WHERE text MATCH jieba_query('中华国歌')",
        [], |row| row.get::<_, i64>(0)
    )?);
    assert_eq!(2, conn.query_row(
        "SELECT id FROM d WHERE text MATCH simple_query('zhoujiel')",
        [], |row| row.get::<_, i64>(0)
    )?);
    Ok(())
}
```


# License

Licensed under MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)


# Version map

This is the compatible version map between `libsimple` and `rusqlite`:

| `libsimple` version | `rusqlite` version |
|---------------------|--------------------|
| =0.3.5              | ~0.33              |
| =0.3.4              | ~0.32              |
| =0.3.3              | ~0.32              |
| =0.3.2              | ~0.32              |
| =0.3.1              | ~0.32              |
| =0.3.0              | ~0.31              |
| =0.2.2              | ~0.31              |
| =0.2.1              | ~0.31              |
| =0.2.0              | ~0.31              |
| =0.1.0              | ~0.31              |


# Generate CMRC

This is only required when the `pinyin.txt` updated.
Normal user can ignore this.

```bash
cd simple && mkdir build && cd build
cmake .. -DBUILD_SQLITE3=off -DSIMPLE_WITH_JIEBA=off -DBUILD_TEST_EXAMPLE=off
make
cp -f _cmrc/include/cmrc/cmrc.hpp ../../cmrc/include/cmrc/cmrc.hpp
cp -f __cmrc_PINYIN_TEXT/lib.cpp ../../cmrc/pinyin.txt/lib.cpp
cp -f __cmrc_PINYIN_TEXT/intermediate/contrib/pinyin.txt.cpp ../../cmrc/pinyin.txt/pinyin.txt.cpp
cd .. && rm -r build && cd ..
```
