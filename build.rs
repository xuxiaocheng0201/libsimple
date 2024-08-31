fn main() {
    let mut cfg = cc::Build::new();

    cfg.include("simple/src");
    cfg.file("simple/src/entry.cc");
    cfg.file("simple/src/pinyin.cc");
    cfg.file("simple/src/simple_highlight.cc");
    cfg.file("simple/src/simple_tokenizer.cc");

    cfg.include("simple/contrib/sqlite3");

    cfg.include("cmrc/include");
    cfg.file("cmrc/pinyin.txt/lib.cpp");
    cfg.file("cmrc/pinyin.txt/pinyin.txt.cpp");

    if cfg!(feature = "jieba") {
        cfg.define("USE_JIEBA", "1");
        cfg.include("cppjieba/include");
        cfg.include("cppjieba/deps/limonp/include");
    }

    cfg.flag("-Wno-comment");
    cfg.flag("-Wno-unused-parameter");
    cfg.flag("-Wno-deprecated-declarations");
    cfg.flag("-Wno-unused-but-set-variable");

    cfg.cpp(true);
    cfg.std("c++14");
    cfg.flag_if_supported("/utf-8");
    cfg.compile("simple");
}
