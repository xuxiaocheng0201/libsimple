fn main() {
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
    // https://github.com/wangfenjin/simple/pull/203
    if !std::path::Path::new("patches/simple-203.patch.patched").exists() {
        println!("cargo:rerun-if-changed=patches/simple-203.patch");
        run(
            std::process::Command::new(std::env::var("PATCH").unwrap_or("patch".to_string()))
                .arg("-p1")
                .arg("-d").arg("simple")
                .arg("-i").arg("../patches/simple-203.patch")
                .arg("--no-backup-if-mismatch"),
            "patch",
        );
        std::fs::write("patches/simple-203.patch.patched", "").unwrap();
    }
    // https://github.com/wangfenjin/simple/pull/203#issuecomment-4047703248
    if !std::path::Path::new("patches/simple-203-2.patch.patched").exists() {
        println!("cargo:rerun-if-changed=patches/simple-203-2.patch");
        run(
            std::process::Command::new(std::env::var("PATCH").unwrap_or("patch".to_string()))
                .arg("-p1")
                .arg("-d").arg("simple")
                .arg("-i").arg("../patches/simple-203-2.patch")
                .arg("--no-backup-if-mismatch"),
            "patch",
        );
        std::fs::write("patches/simple-203-2.patch.patched", "").unwrap();
    }
}

fn generate_cmrc() -> std::path::PathBuf {
    // https://github.com/rust-lang/cmake-rs/issues/273
    // let mut cfg = cmake::Config::new("simple");
    // cfg.define("BUILD_SQLITE3", "off");
    // cfg.define("SIMPLE_WITH_JIEBA", "off");
    // cfg.define("BUILD_TEST_EXAMPLE", "off");
    // cfg.generator("Unix Makefiles");
    // cfg.build();

    let mut out_dir = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
    out_dir.push("build");
    let out_dir = out_dir;

    run(
        std::process::Command::new(std::env::var("CMAKE").unwrap_or("cmake".to_string()))
            .arg("simple")
            .arg("-B").arg(&out_dir)
            .arg("-DBUILD_SQLITE3=off")
            .arg("-DSIMPLE_WITH_JIEBA=off")
            .arg("-DBUILD_TEST_EXAMPLE=off")
            .arg("-G").arg("Unix Makefiles"),
        "cmake",
    );
    run(
        std::process::Command::new(std::env::var("MAKE").unwrap_or("make".to_string()))
            .current_dir(&out_dir),
        "make",
    );

    out_dir
}

fn compile_simple(cmrc_dir: std::path::PathBuf) {
    let mut cfg = cc::Build::new();

    cfg.include("simple/src");
    cfg.file("simple/src/entry.cc");
    cfg.file("simple/src/pinyin.cc");
    cfg.file("simple/src/simple_highlight.cc");
    cfg.file("simple/src/simple_tokenizer.cc");

    cfg.include("simple/contrib/sqlite3");

    cfg.include(cmrc_dir.join("_cmrc/include"));
    cfg.file(cmrc_dir.join("__cmrc_PINYIN_TEXT/lib.cpp"));
    cfg.file(cmrc_dir.join("__cmrc_PINYIN_TEXT/intermediate/contrib/pinyin.txt.cpp"));

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
