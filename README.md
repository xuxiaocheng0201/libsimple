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
libsimple = "~0.9"
```


# Example

```rust
use anyhow::Result;
use tempfile::tempdir;

fn main() -> Result<()> {
    libsimple::enable_auto_extension()?;
    let dir = tempdir()?;
    libsimple::release_jieba_dict(&dir)?;
    
    let conn = rusqlite::Connection::open_in_memory()?;
    libsimple::set_jieba_dict(&conn, &dir)?;
    
    conn.execute_batch("
        CREATE VIRTUAL TABLE d USING fts5(id, text, tokenize = 'simple');
        INSERT INTO d (id, text) VALUES (1, '中华人民共和国国歌');
        INSERT INTO d (id, text) VALUES (2, '周杰伦');
    ")?;
    assert_eq!(1, conn.query_row(
        "SELECT id FROM d WHERE text MATCH jieba_query('中华国歌')",
        [], |row| row.get::<_, i64>(0),
    )?);
    assert_eq!(2, conn.query_row(
        "SELECT id FROM d WHERE text MATCH simple_query('zhoujiel')",
        [], |row| row.get::<_, i64>(0),
    )?);
    Ok(())
}
```


# Patches

Note that since version 0.9.0, `sqlite3_simple_init` has been split into `sqlite3_simple_init` and `sqlite3_simpletokenizer_init`.

This is a fix for users who use SqlCipher.
If the fts5 tokenizer is loaded by `sqlite3_auto_extension`,
users will not have the opportunity to set the key,
resulting in `automatic extension loading failed: file is not a database`.

After applying this:
```rust
# fn main() -> rusqlite::Result<()> {
unsafe fn enable_extension(connection: &rusqlite::Connection, func: rusqlite::auto_extension::RawAutoExtension) -> rusqlite::Result<()> {
    let mut err = std::ptr::null_mut();
    let res = unsafe { func(connection.handle(), &mut err, std::ptr::null()) };
    if res != rusqlite::ffi::SQLITE_OK {
        Err(if err.is_null() {
            rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(res), None)
        } else {
            let c_str = unsafe { std::ffi::CStr::from_ptr(err) };
            let s = c_str.to_string_lossy().into_owned();
            unsafe { rusqlite::ffi::sqlite3_free(err as _) };
            rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(res), Some(s))
        })
    } else {
        Ok(())
    }
}
unsafe { rusqlite::auto_extension::register_auto_extension(libsimple::ffi::sqlite3_simple_init) }?;
let connection = rusqlite::Connection::open_in_memory()?;
connection.pragma_update(None, "key", "123123123")?;
connection.pragma_update(None, "journal_mode", "WAL")?;
unsafe { enable_extension(&connection, libsimple::ffi::sqlite3_simpletokenizer_init) }?;
// use simple normally
# Ok(())
# }
```


# License

Licensed under MIT license ([LICENSE](LICENSE) or <http://opensource.org/licenses/MIT>)


# Version map

This is the compatible version map between `libsimple` and `rusqlite`:

| `libsimple` version | `rusqlite` version |
|---------------------|--------------------|
| =0.9.0              | >=0.32,<1.0        |
| =0.8.0              | >=0.32,<1.0        |
| =0.7.1              | >=0.32,<1.0        |
| =0.7.0              | >=0.32,<1.0        |
| =0.6.1              | >=0.32,<1.0        |
| =0.6.0              | >=0.32,<1.0        |
| =0.5.0              | ~0.35              |
| =0.4.0              | ~0.34              |
| =0.3.7              | ~0.34              |
| =0.3.6              | ~0.33              |
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
