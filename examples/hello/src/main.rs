use anyhow::Result;
use tempfile::tempdir;
use rusqlite;

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
    assert_eq!(1, conn
        .query_row("SELECT id FROM d WHERE text MATCH jieba_query('中华国歌')", [], |row| row.get::<_, i64>(0))?
    );
    assert_eq!(2, conn
        .query_row("SELECT id FROM d WHERE text MATCH simple_query('zhoujiel')", [], |row| row.get::<_, i64>(0))?
    );
    Ok(())
}
