#[test]
fn test() -> anyhow::Result<()> {
    libsimple::enable_auto_extension()?;
    let file = tempfile::NamedTempFile::new()?;
    std::fs::write(&file, "U+5468: zhōu # 周\nU+4F26: lún  # 伦")?;
    let conn = rusqlite::Connection::open_in_memory()?;
    libsimple::set_pinyin_dict(&conn, &file)?;
    conn.execute_batch("
        CREATE VIRTUAL TABLE d USING fts5(x, tokenize = 'simple');
        INSERT INTO d(x) values ('周杰伦');
    ")?;
    assert_eq!(1, conn.query_row(
        "SELECT COUNT(*) FROM d WHERE x match simple_query('zhou')",
        [], |row| row.get::<_, i64>(0),
    )?);
    assert_eq!(1, conn.query_row(
        "SELECT COUNT(*) FROM d WHERE x match simple_query('lun')",
        [], |row| row.get::<_, i64>(0),
    )?);
    assert_eq!(0, conn.query_row(
        "SELECT COUNT(*) FROM d WHERE x match simple_query('jie')",
        [], |row| row.get::<_, i64>(0),
    )?);
    assert_eq!(1, conn.query_row(
        "SELECT COUNT(*) FROM d WHERE x match simple_query('zhou lun')",
        [], |row| row.get::<_, i64>(0),
    )?);
    libsimple::set_pinyin_dict(&conn, "")?; // Use embedded pinyin.txt
    conn.execute_batch("INSERT INTO d(d) VALUES('rebuild');")?;
    assert_eq!(1, conn.query_row(
        "SELECT COUNT(*) FROM d WHERE x match simple_query('jie')",
        [], |row| row.get::<_, i64>(0),
    )?);
    Ok(())
}
