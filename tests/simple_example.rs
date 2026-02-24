#[test]
fn test() -> anyhow::Result<()> {
    libsimple::enable_auto_extension()?;
    let conn = rusqlite::Connection::open_in_memory()?;
    conn.execute_batch(r#"
        CREATE VIRTUAL TABLE t1 USING fts5(x, tokenize = 'simple');
        INSERT INTO t1(x) VALUES
            ('周杰伦 Jay Chou:最美的不是下雨天，是曾与你躲过雨的屋檐'),
            ('I love China! 我爱中国!'),
            ('@English &special _characters."''bacon-&and''-eggs%');
    "#)?;
    // Special characters
    assert_eq!(r#"[@]English [&]special [_]characters[."']bacon[-&]and['-]eggs[%]"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('@"''._-&%');"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"@English &special _characters.["']bacon-&and[']-eggs%"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('"''"');"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"@English &special _characters."[']bacon-&and[']-eggs%"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH '"''"';"#,
        [], |row| row.get::<_, String>(0),
    )?);
    // Highlight and snippet
    assert_eq!(r#"周[杰伦] Jay Chou:最美的不是下雨天，是曾与你躲过雨的屋檐"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('杰伦')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"...[杰]..."#, conn.query_row(
        r#"SELECT simple_snippet(t1, 0, '[', ']', '...', 1) FROM t1 WHERE x MATCH simple_query('杰伦')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"...[杰伦]..."#, conn.query_row(
        r#"SELECT simple_snippet(t1, 0, '[', ']', '...', 2) FROM t1 WHERE x MATCH simple_query('杰伦')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"周[杰伦]..."#, conn.query_row(
        r#"SELECT simple_snippet(t1, 0, '[', ']', '...', 3) FROM t1 WHERE x MATCH simple_query('杰伦')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"周[杰伦] Jay..."#, conn.query_row(
        r#"SELECT simple_snippet(t1, 0, '[', ']', '...', 4) FROM t1 WHERE x MATCH simple_query('杰伦')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"周[杰伦] Jay Chou..."#, conn.query_row(
        r#"SELECT simple_snippet(t1, 0, '[', ']', '...', 5) FROM t1 WHERE x MATCH simple_query('杰伦')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"3,9;"#, conn.query_row(
        r#"SELECT simple_highlight_pos(t1, 0) FROM t1 WHERE x MATCH simple_query('杰伦')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"...[雨天]，是曾与你躲过[雨]..."#, conn.query_row(
        r#"SELECT simple_snippet(t1, 0, '[', ']', '...', 10) FROM t1 WHERE x MATCH simple_query('雨天')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"37,43;64,67;"#, conn.query_row(
        r#"SELECT simple_highlight_pos(t1, 0) FROM t1 WHERE x MATCH simple_query('雨天')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    // Pinyin
    assert_eq!(r#"[周杰伦] Jay Chou:最美的不是下雨天，是曾与你躲过雨的屋檐"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('zhoujiel')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"0,9;"#, conn.query_row(
        r#"SELECT simple_highlight_pos(t1, 0) FROM t1 WHERE x MATCH simple_query('zhoujiel')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"[周杰]伦 Jay Chou:最美的不是下雨天，是曾与你躲过雨的屋檐"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('zhoujie')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(None, rusqlite::OptionalExtension::optional(conn.query_row(
        r#"SELECT 1 FROM t1 WHERE x MATCH simple_query('jiezhou')"#,
        [], |row| row.get::<_, String>(0),
    ))?);
    assert_eq!(r#"[周杰伦] Jay Chou:最美的不是下雨天，是曾与你躲过雨的屋檐"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('zjl')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"[周杰]伦 Jay Chou:最美的不是下雨天，是曾与你躲过雨的屋檐"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('ZHOUJI')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"I [love] China! 我爱[中国]!"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('love zg')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(r#"周[杰伦 Jay] Chou:最美的不是下雨天，是曾与你躲过雨的屋檐"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('jl J')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    // Jieba
    let dir = tempfile::tempdir()?;
    libsimple::release_jieba_dict(&dir)?;
    libsimple::set_jieba_dict(&conn, &dir)?;
    assert_eq!(r#"I love China! [我爱中国]!"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH simple_query('国中woai')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    assert_eq!(None, rusqlite::OptionalExtension::optional(conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH jieba_query('国中woai')"#,
        [], |row| row.get::<_, String>(0),
    ))?);
    assert_eq!(r#"I love China! [我爱中国]!"#, conn.query_row(
        r#"SELECT simple_highlight(t1, 0, '[', ']') FROM t1 WHERE x MATCH jieba_query('中国woai')"#,
        [], |row| row.get::<_, String>(0),
    )?);
    Ok(())
}
