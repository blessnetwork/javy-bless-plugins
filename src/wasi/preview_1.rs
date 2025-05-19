#[link(wasm_import_module = "wasi_snapshot_preview1")]
unsafe extern "C" {
    #[link_name = "path_open"]
    pub unsafe fn path_open(
        dirfd: i32,
        dirflags: i32,
        path: i32,
        path_len: i32,
        oflags: i32,
        fs_rights_base: i64,
        _fs_rights_inheriting: i64,
        fdflags: i32,
        fd_ptr: i32,
   ) -> i32;

   #[link_name = "fd_close"]
    pub unsafe fn fd_close(
       fd: i32,
   ) -> i32;

    #[link_name = "fd_read"]
     pub unsafe fn fd_read(
         fd: i32,
         iovs: i32,
         iovs_len: i32,
         nread: i32,
    ) -> i32;

    #[link_name = "fd_prestat_get"]
    pub unsafe fn fd_prestat_get(
        fd: i32,
        path_len: i32,
    ) -> i32;

    #[link_name = "fd_prestat_dir_name"]
    pub unsafe fn fd_prestat_dir_name(
        fd: i32,
        path_ptr: i32,
        path_len: i32,
    ) -> i32;

}
