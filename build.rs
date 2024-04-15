use std::env::var;
use std::fs::{copy, create_dir, create_dir_all, read_dir};
use std::path::Path;
use std::process::Command;

use anyhow::{anyhow, Result};

fn copy_dir<U: AsRef<Path>, V: AsRef<Path>>(from_dir: U, to_dir: V) -> Result<()> {
    let from_dir = from_dir.as_ref();
    let to_dir = to_dir.as_ref();
    if !to_dir.is_dir() {
        create_dir_all(to_dir)?;
    }
    for entry in read_dir(from_dir)? {
        let path = entry?.path();
        let to_path = to_dir.join(path.file_name().unwrap());
        if path.is_dir() {
            copy_dir(path, to_path)?;
        } else {
            copy(path, to_path)?;
        }
    }
    Ok(())
}

fn execute(commond: &mut Command) -> Result<()> {
    commond.spawn()?.wait()?.success().then_some(())
        .ok_or_else(|| anyhow!("failed to execute command: {:?}", commond))
}

fn main() -> Result<()> {
    let out_dir = var("OUT_DIR")?;

    copy_dir("simple", &Path::new(&out_dir).join("simple"))?;

    let build_dir = Path::new(&out_dir).join("build");
    if !build_dir.is_dir() {
        create_dir(&build_dir)?;
    }
    execute(Command::new("cmake").arg("../simple")
        .current_dir(&build_dir)
        .args(&["-G", "Unix Makefiles"])
        .arg("-DCMAKE_INSTALL_PREFIX=release")
        .arg("-DCODE_COVERAGE=OFF")
        .arg("-DBUILD_SHELL=OFF")
        .arg("-DBUILD_TEST_EXAMPLE=OFF")
    )?;
    execute(Command::new("make").arg("install")
        .current_dir(&build_dir)
    )?;

    Ok(())
}
