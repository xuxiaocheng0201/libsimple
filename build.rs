fn main() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    dircpy::CopyBuilder::new("simple", out_dir.join("simple"))
        .overwrite(true)
        .run().expect("failed to copy source simple");
    dircpy::CopyBuilder::new("cppjieba", out_dir.join("cppjieba"))
        .overwrite(true)
        .run().expect("failed to copy source cppjieba");
    apply_patch();

    let cmrc_dir = generate_cmrc();
    compile_simple(cmrc_dir);
}

// https://docs.rs/crate/cmake/0.1.57/source/src/lib.rs#1091-1115
fn run(cmd: &mut std::process::Command, program: &str) {
    eprintln!("running: {:?}", cmd);
    let status = match cmd.status() {
        Ok(status) => status,
        Err(ref e) if e.kind() == std::io::ErrorKind::NotFound =>
            panic!("failed to execute command: {e}\nis `{program}` not installed?"),
        Err(e) => panic!("failed to execute command: {e}"),
    };
    if !status.success() {
        if status.code() == Some(127) {
            panic!("command did not execute successfully, got: {status}, is `{program}` not installed?");
        }
        panic!("command did not execute successfully, got: {status}");
    }
}

fn apply_patch() {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    println!("cargo:rerun-if-changed=patches/simple-203.patch");
    run(
        std::process::Command::new(std::env::var("PATCH").unwrap_or("patch".to_string()))
            .arg("-p1").arg("--no-backup-if-mismatch")
            .arg("-d").arg(out_dir.join("simple"))
            .stdin(std::fs::File::open("patches/simple-203.patch").unwrap()),
        "patch",
    );
    println!("cargo:rerun-if-changed=patches/simple-203-2.patch");
    run(
        std::process::Command::new(std::env::var("PATCH").unwrap_or("patch".to_string()))
            .arg("-p1").arg("--no-backup-if-mismatch")
            .arg("-d").arg(out_dir.join("simple"))
            .stdin(std::fs::File::open("patches/simple-203-2.patch").unwrap()),
        "patch",
    );
}

fn generate_cmrc() -> std::path::PathBuf {
    // https://github.com/rust-lang/cmake-rs/issues/273
    // let mut cfg = cmake::Config::new("simple");
    // cfg.define("BUILD_SQLITE3", "off");
    // cfg.define("SIMPLE_WITH_JIEBA", "off");
    // cfg.define("BUILD_TEST_EXAMPLE", "off");
    // cfg.generator("Unix Makefiles");
    // cfg.build();

    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    run(
        std::process::Command::new(std::env::var("CMAKE").unwrap_or("cmake".to_string()))
            .arg(out_dir.join("simple"))
            .arg("-B").arg(out_dir.join("build"))
            .arg("-DBUILD_SQLITE3=off")
            .arg("-DSIMPLE_WITH_JIEBA=off")
            .arg("-DBUILD_TEST_EXAMPLE=off")
            .arg("-G").arg("Unix Makefiles"),
        "cmake",
    );
    run(
        std::process::Command::new(std::env::var("MAKE").unwrap_or("make".to_string()))
            .current_dir(out_dir.join("build")),
        "make",
    );

    out_dir.join("build")
}

fn compile_simple(cmrc_dir: std::path::PathBuf) {
    let out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());

    let mut cfg = cc::Build::new();

    cfg.include(out_dir.join("simple/src"));
    cfg.file(out_dir.join("simple/src/entry.cc"));
    cfg.file(out_dir.join("simple/src/pinyin.cc"));
    cfg.file(out_dir.join("simple/src/simple_highlight.cc"));
    cfg.file(out_dir.join("simple/src/simple_tokenizer.cc"));

    cfg.include(out_dir.join("simple/contrib/sqlite3"));

    cfg.include(cmrc_dir.join("_cmrc/include"));
    cfg.file(cmrc_dir.join("__cmrc_PINYIN_TEXT/lib.cpp"));
    cfg.file(cmrc_dir.join("__cmrc_PINYIN_TEXT/intermediate/contrib/pinyin.txt.cpp"));

    if cfg!(feature = "jieba") {
        cfg.define("USE_JIEBA", "1");
        cfg.include(out_dir.join("cppjieba/include"));
        cfg.include(out_dir.join("cppjieba/deps/limonp/include"));
    }

    cfg.flag_if_supported("-Wno-comment");
    cfg.flag_if_supported("-Wno-unused-parameter");
    cfg.flag_if_supported("-Wno-deprecated-declarations");

    cfg.cpp(true);
    cfg.std("c++14");
    cfg.flag_if_supported("/utf-8");
    cfg.compile("simple");
}
