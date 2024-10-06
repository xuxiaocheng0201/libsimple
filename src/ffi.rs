//! Raw FFI bindings to simple.

use std::ffi::{c_char, c_int, CStr};

use rusqlite::{Error, ffi, Result};

extern "C" {
    /// The entrypoint for the [simple](https://github.com/wangfenjin/simple) extension.
    pub fn sqlite3_simple_init(db: *mut ffi::sqlite3, pz_err_msg: *mut *mut c_char, p_api: *const ffi::sqlite3_api_routines) -> c_int;


    /// The sqlite function entrypoint for `simple_query` function.
    pub fn simple_query(ctx: *mut ffi::sqlite3_context, argc: c_int, argv: *mut *mut ffi::sqlite3_value);

    #[cfg(feature = "jieba")]
    #[cfg_attr(docsrs, doc(cfg(feature = "jieba")))]
    /// The sqlite function entrypoint for `jieba_dict` function.
    pub fn jieba_dict(ctx: *mut ffi::sqlite3_context, argc: c_int, argv: *mut *mut ffi::sqlite3_value);

    #[cfg(feature = "jieba")]
    #[cfg_attr(docsrs, doc(cfg(feature = "jieba")))]
    /// The sqlite function entrypoint for `jieba_query` function.
    pub fn jieba_query(ctx: *mut ffi::sqlite3_context, argc: c_int, argv: *mut *mut ffi::sqlite3_value);


    /// The sqlite fts5 function entrypoint for `simple_highlight` function.
    pub fn simple_highlight(api: *const ffi::Fts5ExtensionApi, fts: *mut ffi::Fts5Context, ctx: *mut ffi::sqlite3_context, argc: c_int, argv: *mut *mut ffi::sqlite3_value);

    /// The sqlite fts5 function entrypoint for `simple_highlight_pos` function.
    pub fn simple_highlight_pos(api: *const ffi::Fts5ExtensionApi, fts: *mut ffi::Fts5Context, ctx: *mut ffi::sqlite3_context, argc: c_int, argv: *mut *mut ffi::sqlite3_value);

    /// The sqlite fts5 function entrypoint for `simple_snippet` function.
    pub fn simple_snippet(api: *const ffi::Fts5ExtensionApi, fts: *mut ffi::Fts5Context, ctx: *mut ffi::sqlite3_context, argc: c_int, argv: *mut *mut ffi::sqlite3_value);
}

/// This is a re-exported and enhanced version of [`rusqlite::error::check(res)`](rusqlite::error::check)
#[doc(hidden)]
pub fn check_err(res: c_int) -> Result<()> {
    if res == ffi::SQLITE_OK {
        return Ok(());
    }
    let err = unsafe { ffi::sqlite3_errstr(res) };
    if err.is_null() {
        return Err(Error::SqliteFailure(ffi::Error::new(res), None));
    }
    let msg = unsafe { CStr::from_ptr(err) }.to_str()?;
    Err(Error::SqliteFailure(ffi::Error::new(res), Some(msg.to_string())))
}
