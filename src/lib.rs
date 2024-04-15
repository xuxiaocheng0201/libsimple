use std::fs::OpenOptions;
use std::io::Write;
use std::sync::RwLock;

use rusqlite::Connection;
use tempfile::TempPath;

#[doc(hidden)]
#[inline]
pub fn get_dylib() -> &'static [u8] {
    include_bytes!(concat!(env!("OUT_DIR"), "/build/release/bin/libsimple"))
}

static DYLIB: RwLock<Option<TempPath>> = RwLock::new(None);

/// Release the dylib into a temporary file.
pub fn initialize() -> std::io::Result<()> {
    let mut guard = DYLIB.write().unwrap();
    if guard.is_none() {
        let path = tempfile::NamedTempFile::new()?.into_temp_path();
        OpenOptions::new().write(true).create(true).truncate(true).open(&path)?
            .write_all(get_dylib())?;
        guard.replace(path);
    }
    Ok(())
}

/// Delete the temporary dylib file.
pub fn uninitialize() {
    DYLIB.write().unwrap().take();
}

/// # Panic
/// If [`initialize`] is not called or [`uninitialize`] is called.
pub unsafe fn load(connection: &mut Connection) -> rusqlite::Result<()> {
    let guard = DYLIB.read().unwrap();
    let path = guard.as_ref().unwrap();
    connection.load_extension(path, None)
}
