use std::env;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let mut cfg = cc::Build::new();

    cfg.include("simple/src");
    cfg.file("simple/src/pinyin.h");
    cfg.file("simple/src/simple_highlight.h");
    cfg.file("simple/src/simple_tokenizer.h");
    cfg.file("simple/src/pinyin.cc");
    cfg.file("simple/src/simple_highlight.cc");
    cfg.file("simple/src/simple_tokenizer.cc");
    cfg.file("simple/src/entry.cc");

    cfg.include("simple/contrib/sqlite3");

    cfg.include("cmrc/include");
    cfg.file("cmrc/pinyin.txt/lib.cpp");
    cfg.file("cmrc/pinyin.txt/pinyin.txt.cpp");

    if cfg!(feature = "jieba") {
        cfg
            .define("USE_JIEBA", "1")
            .include("cppjieba/include")
            .include("cppjieba/deps/limonp/include");
    }

    cfg
        .cpp(true)
        .std("c++14")
        .flag_if_supported("/utf-8");
    cfg.compile("simple");
    println!("cargo:lib_dir={out_dir}");
}
