//! Raw FFI bindings to simple.

use std::ffi::{c_char, c_int};

use rusqlite::ffi;

unsafe extern "C" {
    /// The entrypoint for the [simple](https://github.com/wangfenjin/simple) extension.
    pub fn sqlite3_simple_init(
        db: *mut ffi::sqlite3,
        pz_err_msg: *mut *mut c_char,
        p_api: *const ffi::sqlite3_api_routines,
    ) -> c_int;


    /// The sqlite function entrypoint for `simple_query` function.
    pub fn simple_query(
        ctx: *mut ffi::sqlite3_context,
        argc: c_int,
        argv: *mut *mut ffi::sqlite3_value,
    );

    /// The sqlite function entrypoint for `pinyin_dict` function.
    pub fn pinyin_dict(
        ctx: *mut ffi::sqlite3_context,
        argc: c_int,
        argv: *mut *mut ffi::sqlite3_value,
    );

    #[cfg(feature = "jieba")]
    /// The sqlite function entrypoint for `jieba_dict` function.
    pub fn jieba_dict(
        ctx: *mut ffi::sqlite3_context,
        argc: c_int,
        argv: *mut *mut ffi::sqlite3_value,
    );

    #[cfg(feature = "jieba")]
    /// The sqlite function entrypoint for `jieba_query` function.
    pub fn jieba_query(
        ctx: *mut ffi::sqlite3_context,
        argc: c_int,
        argv: *mut *mut ffi::sqlite3_value,
    );

    /// The sqlite fts5 function entrypoint for `simple_highlight` function.
    pub fn simple_highlight(
        api: *const ffi::Fts5ExtensionApi,
        fts: *mut ffi::Fts5Context,
        ctx: *mut ffi::sqlite3_context,
        argc: c_int,
        argv: *mut *mut ffi::sqlite3_value,
    );

    /// The sqlite fts5 function entrypoint for `simple_highlight_pos` function.
    pub fn simple_highlight_pos(
        api: *const ffi::Fts5ExtensionApi,
        fts: *mut ffi::Fts5Context,
        ctx: *mut ffi::sqlite3_context,
        argc: c_int,
        argv: *mut *mut ffi::sqlite3_value,
    );

    /// The sqlite fts5 function entrypoint for `simple_snippet` function.
    pub fn simple_snippet(
        api: *const ffi::Fts5ExtensionApi,
        fts: *mut ffi::Fts5Context,
        ctx: *mut ffi::sqlite3_context,
        argc: c_int,
        argv: *mut *mut ffi::sqlite3_value,
    );


    /// The sqlite fts5 tokenizer entrypoint for `fts5_simple_xCreate` function.
    pub fn fts5_simple_xCreate(
        sqlite3: *mut std::ffi::c_void,
        az_arg: *mut *const c_char,
        n_arg: c_int,
        pp_out: *mut *mut ffi::Fts5Tokenizer,
    ) -> c_int;

    /// The sqlite fts5 tokenizer entrypoint for `fts5_simple_xTokenize` function.
    pub fn fts5_simple_xTokenize(
        tokenizer_ptr: *mut ffi::Fts5Tokenizer,
        p_ctx: *mut std::ffi::c_void,
        flags: c_int,
        p_text: *const c_char,
        n_text: c_int,
        x_token: Option<unsafe extern "C" fn (
            p_ctx: *mut std::ffi::c_void,
            t_flags: c_int,
            p_token: *const c_char,
            n_token: c_int,
            i_start: c_int,
            i_end: c_int,
        ) -> c_int>,
    ) -> c_int;

    /// The sqlite fts5 tokenizer entrypoint for `fts5_simple_xDelete` function.
    pub fn fts5_simple_xDelete(
        tokenizer_ptr: *mut ffi::Fts5Tokenizer,
    );
}
