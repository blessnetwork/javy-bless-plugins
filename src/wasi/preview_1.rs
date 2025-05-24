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

   #[link_name = "path_create_directory"]
    pub unsafe fn path_create_directory(
        dirfd: i32,
        path_ptr: i32,
        path_len: i32,
   ) -> i32;

   #[link_name = "path_remove_directory"]
    pub unsafe fn path_remove_directory(
        dirfd: i32,
        path_ptr: i32,
        path_len: i32,
   ) -> i32;


   #[link_name = "path_unlink_file"]
    pub unsafe fn path_unlink_file(
        dirfd: i32,
        path_ptr: i32,
        path_len: i32,
   ) -> i32;

   #[link_name = "path_symlink"]
    pub unsafe fn path_symlink(
        old_path_ptr: i32,
        old_path_len: i32,
        dirfd: i32,
        new_path_ptr: i32,
        new_path_len: i32,
    ) -> i32;

   #[link_name = "fd_close"]
    pub unsafe fn fd_close(
       fd: i32,
   ) -> i32;

    #[link_name = "fd_prestat_get"]
    pub unsafe fn fd_prestat_get(
        fd: i32,
        path_len: i32,
    ) -> i32;

    #[link_name = "fd_read"]
    pub unsafe fn fd_read(
        fd: i32,
        iovec_slice: i32,
        iovec_len: i32,
        readn_ptr: i32,
    ) -> i32;

    #[link_name = "fd_write"]
    pub unsafe fn fd_write(
        fd: i32,
        iovec_slice: i32,
        iovec_len: i32,
        writen_ptr: i32,
    ) -> i32;

    #[link_name = "fd_prestat_dir_name"]
    pub unsafe fn fd_prestat_dir_name(
        fd: i32,
        path_ptr: i32,
        path_len: i32,
    ) -> i32;

    #[link_name = "path_link"]
    pub unsafe fn path_link(
        old_dirfd: i32,
        fd_lookup_flags: i32,
        old_path: i32,
        old_path_len: i32,
        new_dirfd: i32,
        new_path: i32,
        new_path_len: i32,
    ) -> i32;

    #[link_name = "path_rename"]
    pub unsafe fn path_rename(
        old_dirfd: i32,
        old_path: i32,
        old_path_len: i32,
        new_dirfd: i32,
        new_path: i32,
        new_path_len: i32,
    ) -> i32;

    

    #[link_name = "path_filestat_get"]
    pub unsafe fn path_filestat_get(
        dirfd: i32,
        flags: i32,
        path: i32,
        path_len: i32,
        stat_ptr: i32,
    ) -> i32;
}
