#![doc = include_str!("../README.md")]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![warn(missing_docs)]

pub mod ffi;

/// Enable sqlite3_simple_init() as an auto extension.
#[inline]
pub fn enable_auto_extension() -> rusqlite::Result<()> {
    unsafe { rusqlite::auto_extension::register_auto_extension(ffi::sqlite3_simple_init) }
}

/// Disable sqlite3_simple_init() as an auto extension.
#[inline]
pub fn disable_auto_extension() -> rusqlite::Result<()> {
    rusqlite::auto_extension::cancel_auto_extension(ffi::sqlite3_simple_init);
    Ok(())
}

/// Use custom `pinyin.txt`.
/// Only need to call once for each process.
///
/// Call `set_pinyin_dict(&conn, "")` to use embedded `pinyin.txt`.
/// # Notice
/// It is recommended to call pinyin_dict() once before building the index and querying.
/// If the pinyin mapping is replaced, the existing index will not be automatically rebuilt,
/// and manual index reconstruction is required.
pub fn set_pinyin_dict(connection: &rusqlite::Connection, file: impl AsRef<std::path::Path>) -> rusqlite::Result<()> {
    let file = file.as_ref();
    let file = file.to_str().ok_or_else(|| rusqlite::Error::InvalidPath(file.to_path_buf()))?;
    connection.query_row("SELECT pinyin_dict(?)", rusqlite::params![file], |_| Ok(()))
}

/// Release dict files into directory.
/// Only need to call this method once.
///
/// Then you may call [`set_jieba_dict`] for each connection.
#[cfg(feature = "jieba")]
pub fn release_jieba_dict(directory: impl AsRef<std::path::Path>) -> std::io::Result<()> {
    let directory = directory.as_ref().to_path_buf();
    if !directory.is_dir() { std::fs::create_dir_all(&directory)?; }

    macro_rules! embedded_file {
        ($target: ident, $source: expr) => {
            let file = include_bytes!(concat!("../cppjieba/dict/", $source));
            let target = $target.join($source);
            better_embedded::release_file_with_check(
                file, &target,
                better_embedded::strategies::DefaultCheckStrategy::config(),
            )?;
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
/// You should call [`release_jieba_dict`] first.
#[cfg(feature = "jieba")]
pub fn set_jieba_dict(connection: &rusqlite::Connection, directory: impl AsRef<std::path::Path>) -> rusqlite::Result<()> {
    let directory = directory.as_ref();
    let directory = directory.to_str().ok_or_else(|| rusqlite::Error::InvalidPath(directory.to_path_buf()))?;
    connection.query_row("SELECT jieba_dict(?)", rusqlite::params![directory], |_| Ok(()))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_pinyin() -> anyhow::Result<()> {
        crate::enable_auto_extension()?;
        let file = tempfile::NamedTempFile::new()?;
        std::fs::write(&file, "U+5468: zhōu # 周\nU+4F26: lún  # 伦")?;
        let conn = rusqlite::Connection::open_in_memory()?;
        crate::set_pinyin_dict(&conn, &file)?;
        conn.execute_batch("
            CREATE VIRTUAL TABLE d USING fts5(x, tokenize = 'simple');
            INSERT INTO d(x) values ('周杰伦');
        ")?;
        assert_eq!(1, conn.query_row(
           "SELECT COUNT(*) FROM d WHERE x match simple_query('zhou')",
           [], |row| row.get::<_, i64>(0)
        )?);
        assert_eq!(1, conn.query_row(
           "SELECT COUNT(*) FROM d WHERE x match simple_query('lun')",
           [], |row| row.get::<_, i64>(0)
        )?);
        assert_eq!(0, conn.query_row(
           "SELECT COUNT(*) FROM d WHERE x match simple_query('jie')",
           [], |row| row.get::<_, i64>(0)
        )?);
        assert_eq!(1, conn.query_row(
           "SELECT COUNT(*) FROM d WHERE x match simple_query('zhou lun')",
           [], |row| row.get::<_, i64>(0)
        )?);
        crate::set_pinyin_dict(&conn, "")?; // Use embedded pinyin.txt
        conn.execute_batch("INSERT INTO d(d) VALUES('rebuild');")?;
        assert_eq!(1, conn.query_row(
            "SELECT COUNT(*) FROM d WHERE x match simple_query('jie')",
            [], |row| row.get::<_, i64>(0)
        )?);
        Ok(())
    }

    #[test]
    fn test_jieba() -> anyhow::Result<()> {
        crate::enable_auto_extension()?;
        let dir = tempfile::tempdir()?;
        crate::release_jieba_dict(&dir)?;

        let conn = rusqlite::Connection::open_in_memory()?;
        crate::set_jieba_dict(&conn, &dir)?;
        conn.execute_batch("
            PRAGMA key = '123456';
            CREATE TABLE singer (id INTEGER, name TEXT);
            CREATE VIRTUAL TABLE d USING fts5(id, name, tokenize = 'simple');
            CREATE TRIGGER trigger AFTER INSERT ON singer BEGIN
                INSERT INTO d(id, name) VALUES (new.id, new.name);
            END;
            INSERT INTO singer (id, name) VALUES (1, '中华人民共和国国歌');
        ")?;
        assert_eq!(1, conn.query_row(
           "SELECT id FROM d WHERE name MATCH jieba_query('中华国歌')",
           [], |row| row.get::<_, i64>(0)
        )?);
        Ok(())
    }
}
