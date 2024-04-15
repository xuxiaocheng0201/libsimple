use std::fs::create_dir_all;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::RwLock;

use rusqlite::{Connection, params};
use thiserror::Error;

#[cfg(target_os = "windows")]
macro_rules! simple_dylib {
    () => {
        "libsimple.dll"
    };
}
#[cfg(not(target_os = "windows"))]
macro_rules! simple_dylib {
    () => {
        "libsimple.so"
    };
}

#[doc(hidden)]
#[inline]
pub fn get_dylib() -> &'static [u8] {
    include_bytes!(concat!(env!("OUT_DIR"), "/build/release/bin/", simple_dylib!()))
}

macro_rules! embedded_file {
    ($target: ident, $source: expr) => {
        let file = include_bytes!(concat!(env!("OUT_DIR"), "/build/release/bin/", $source));
        let target = $target.join($source);
        if let Some(parent) = target.parent() {
            create_dir_all(parent)?;
        }
        OpenOptions::new().write(true).create(true).truncate(true)
            .open(&target)?.write_all(file)?;
    };
}

static DICT: RwLock<Option<PathBuf>> = RwLock::new(None);

/// Release dylib and dict files into directory.
pub fn initialize(directory: impl AsRef<Path>) -> std::io::Result<()> {
    let directory = directory.as_ref().to_path_buf();
    if !directory.exists() { create_dir_all(&directory)?; }

    embedded_file!(directory, simple_dylib!());
    embedded_file!(directory, "dict/jieba.dict.utf8");
    embedded_file!(directory, "dict/user.dict.utf8");
    embedded_file!(directory, "dict/hmm_model.utf8");
    embedded_file!(directory, "dict/idf.utf8");
    embedded_file!(directory, "dict/stop_words.utf8");

    DICT.write().unwrap().replace(directory);
    Ok(())
}

#[derive(Debug, Error)]
#[error(transparent)]
pub enum Error {
    IoError(#[from] std::io::Error),
    SqliteError(#[from] rusqlite::Error),
}

/// Load the `simple` extension
/// # Panic
/// If [initialize] is not called.
/// Or the directory is removed after the call.
pub fn load(connection: &Connection) -> Result<(), Error> {
    let guard = DICT.read().unwrap();
    let path = guard.as_ref().expect("libsimple is not initialized");
    unsafe { connection.load_extension(path.join("libsimple"), None) }?;
    let dict = path.join("dict");
    let dict = dict.to_str().ok_or_else(|| std::io::Error::new(std::io::ErrorKind::InvalidInput, "invalid path"))?;
    connection.query_row("SELECT jieba_dict(?)", params![dict], |_| Ok(()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::Result;
    use tempfile::tempdir;

    //noinspection SpellCheckingInspection
    #[test]
    fn test() -> Result<()> {
        let dir = tempdir()?;
        crate::initialize(&dir)?;

        let conn = rusqlite::Connection::open_in_memory()?;
        crate::load(&conn)?;
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
