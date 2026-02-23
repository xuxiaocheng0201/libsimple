fn main() {
    let cmrc_dir = generate_cmrc();
    compile_simple(cmrc_dir);
}

fn generate_cmrc() -> std::path::PathBuf {
    let mut cfg = cmake::Config::new("simple");

    cfg.define("BUILD_SQLITE3", "off");
    cfg.define("SIMPLE_WITH_JIEBA", "off");
    cfg.define("BUILD_TEST_EXAMPLE", "off");

    if cfg!(windows) {
        cfg.generator("Unix Makefiles");
    }

    let mut path = cfg.build();
    path.push("build");
    path
}

fn compile_simple(cmrc_dir: std::path::PathBuf) {
    let mut cfg = cc::Build::new();

    cfg.include("simple/src");
    cfg.file("simple/src/entry.cc");
    cfg.file("simple/src/pinyin.cc");
    cfg.file("simple/src/simple_highlight.cc");
    cfg.file("simple/src/simple_tokenizer.cc");

    cfg.include("simple/contrib/sqlite3");

    cfg.include(&cmrc_dir.join("_cmrc/include"));
    cfg.file(&cmrc_dir.join("__cmrc_PINYIN_TEXT/lib.cpp"));
    cfg.file(&cmrc_dir.join("__cmrc_PINYIN_TEXT/intermediate/contrib/pinyin.txt.cpp"));

    if cfg!(feature = "jieba") {
        cfg.define("USE_JIEBA", "1");
        cfg.include("cppjieba/include");
        cfg.include("cppjieba/deps/limonp/include");
    }

    cfg.flag_if_supported("-Wno-comment");
    cfg.flag_if_supported("-Wno-unused-parameter");
    cfg.flag_if_supported("-Wno-deprecated-declarations");

    cfg.cpp(true);
    cfg.std("c++14");
    cfg.flag_if_supported("/utf-8");
    cfg.compile("simple");
}
