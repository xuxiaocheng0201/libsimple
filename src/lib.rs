#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

use std::ffi::CStr;
use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

use rusqlite::{Connection, params};
use rusqlite::ffi::{sqlite3_auto_extension, sqlite3_errstr, SQLITE_OK};

use crate::ffi::sqlite3_simple_init;

pub mod ffi;

/// Enable sqlite3_simple_init() as an auto extension.
pub fn enable_auto_extension() -> rusqlite::Result<()> {
    let res = unsafe { sqlite3_auto_extension(Some(sqlite3_simple_init)) };
    // rusqlite::error::check(res)
    if res == SQLITE_OK {
        return Ok(());
    }
    let err = unsafe { sqlite3_errstr(res) };
    if err.is_null() {
        return Err(rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(res), None));
    }
    let msg = unsafe { CStr::from_ptr(err) }.to_str()?;
    Err(rusqlite::Error::SqliteFailure(rusqlite::ffi::Error::new(res), Some(msg.to_string())))
}

/// Release dict files into directory.
/// Only need to call this method once.
///
/// Then you may call [`set_dict`] for each connection.
#[cfg(feature = "jieba")]
#[cfg_attr(docsrs, doc(cfg(feature = "jieba")))]
pub fn release_dict(directory: impl AsRef<Path>) -> std::io::Result<()> {
    let directory = directory.as_ref().to_path_buf();
    if !directory.is_dir() { create_dir_all(&directory)?; }

    macro_rules! embedded_file {
        ($target: ident, $source: expr) => {
            let file = include_bytes!(concat!("../cppjieba/dict/", $source));
            let target = $target.join($source);
            OpenOptions::new().write(true).create(true).truncate(true)
                .open(&target)?.write_all(file)?;
        };
    }
    embedded_file!(directory, "jieba.dict.utf8");
    embedded_file!(directory, "user.dict.utf8");
    embedded_file!(directory, "hmm_model.utf8");
    embedded_file!(directory, "idf.utf8");
    embedded_file!(directory, "stop_words.utf8");

    Ok(())
}

/// Only need to call once for each connection,
/// but must call this function before using sql `jieba_query`.
///
/// You should call [`release_dict`] first.
#[cfg(feature = "jieba")]
#[cfg_attr(docsrs, doc(cfg(feature = "jieba")))]
pub fn set_dict(connection: &Connection, directory: impl AsRef<Path>) -> rusqlite::Result<()> {
    let directory = directory.as_ref();
    let directory = directory.to_str()
        .ok_or_else(|| rusqlite::Error::InvalidPath(directory.to_path_buf()))?;
    connection.query_row("SELECT jieba_dict(?)", params![directory], |_| Ok(()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() -> anyhow::Result<()> {
        crate::enable_auto_extension()?;
        let dir = tempfile::tempdir()?;
        crate::release_dict(&dir)?;

        let conn = rusqlite::Connection::open_in_memory()?;
        crate::set_dict(&conn, &dir)?;
        conn.execute_batch("
            CREATE TABLE singer (id INTEGER, name TEXT);
            CREATE VIRTUAL TABLE d USING fts5(id, name, tokenize = 'simple');
            CREATE TRIGGER dtrigger AFTER INSERT ON singer BEGIN
                INSERT INTO d(id, name) VALUES (new.id, new.name);
            END;
            INSERT INTO singer (id, name) VALUES (1, '中华人民共和国国歌');
        ")?;
        assert_eq!(conn.query_row("SELECT id FROM d WHERE name MATCH jieba_query('中华国歌')",
                                  [], |row| row.get::<_, i64>(0))?, 1);
        Ok(())
    }
}
