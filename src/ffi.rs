//! Raw FFI bindings to simple.
#![allow(missing_docs)]

use std::ffi::{c_char, c_int};
use rusqlite::ffi::{sqlite3, sqlite3_api_routines};

extern "C" {
    pub fn sqlite3_simple_init(db: *mut sqlite3, pz_err_msg: *mut *const c_char, p_api: *const sqlite3_api_routines) -> c_int;
}
