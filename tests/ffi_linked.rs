#[test]
#[ignore] // These function should not be executed. Just compile.
fn test() {
    use std::ptr::{null, null_mut};
    unsafe {
        libsimple::ffi::sqlite3_simple_init(null_mut(), null_mut(), null_mut());
        libsimple::ffi::sqlite3_simpletokenizer_init(null_mut(), null_mut(), null_mut());

        libsimple::ffi::simple_query(null_mut(), 0, null_mut());
        libsimple::ffi::pinyin_dict(null_mut(), 0, null_mut());
        #[cfg(feature = "jieba")]
        libsimple::ffi::jieba_dict(null_mut(), 0, null_mut());
        #[cfg(feature = "jieba")]
        libsimple::ffi::jieba_query(null_mut(), 0, null_mut());

        libsimple::ffi::simple_highlight(null(), null_mut(), null_mut(), 0, null_mut());
        libsimple::ffi::simple_highlight_pos(null(), null_mut(), null_mut(), 0, null_mut());
        libsimple::ffi::simple_snippet(null(), null_mut(), null_mut(), 0, null_mut());

        libsimple::ffi::fts5_simple_xCreate(null_mut(), null_mut(), 0, null_mut());
        libsimple::ffi::fts5_simple_xTokenize(null_mut(), null_mut(), 0, null(), 0, None);
        libsimple::ffi::fts5_simple_xDelete(null_mut());
    }
}
