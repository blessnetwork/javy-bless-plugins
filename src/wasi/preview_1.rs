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
    pub unsafe fn path_create_directory(dirfd: i32, path_ptr: i32, path_len: i32) -> i32;

    #[link_name = "path_remove_directory"]
    pub unsafe fn path_remove_directory(dirfd: i32, path_ptr: i32, path_len: i32) -> i32;

    #[link_name = "path_unlink_file"]
    pub unsafe fn path_unlink_file(dirfd: i32, path_ptr: i32, path_len: i32) -> i32;

    #[link_name = "path_symlink"]
    pub unsafe fn path_symlink(
        old_path_ptr: i32,
        old_path_len: i32,
        dirfd: i32,
        new_path_ptr: i32,
        new_path_len: i32,
    ) -> i32;

    #[link_name = "fd_close"]
    pub unsafe fn fd_close(fd: i32) -> i32;

    #[link_name = "fd_sync"]
    pub unsafe fn fd_sync(fd: i32) -> i32;

    #[link_name = "fd_datasync"]
    pub unsafe fn fd_datasync(fd: i32) -> i32;

    #[link_name = "fd_prestat_get"]
    pub unsafe fn fd_prestat_get(fd: i32, path_len_ptr: i32) -> i32;

    #[link_name = "fd_read"]
    pub unsafe fn fd_read(fd: i32, iovec_slice: i32, iovec_len: i32, readn_ptr: i32) -> i32;

    #[link_name = "fd_write"]
    pub unsafe fn fd_write(fd: i32, iovec_slice: i32, iovec_len: i32, writen_ptr: i32) -> i32;

    #[link_name = "fd_prestat_dir_name"]
    pub unsafe fn fd_prestat_dir_name(fd: i32, path_ptr: i32, path_len: i32) -> i32;

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

    #[link_name = "fd_advise"]
    pub unsafe fn fd_advise(fd: i32, offset: u64, len: u64, advice: i32) -> i32;

    #[link_name = "fd_seek"]
    pub unsafe fn fd_seek(fd: i32, offset: u64, whence: i32, fsize: i32) -> i32;

    #[link_name = "fd_allocate"]
    pub unsafe fn fd_allocate(fd: i32, offset: u64, len: u64) -> i32;

    #[link_name = "fd_filestat_get"]
    pub unsafe fn fd_filestat_get(fd: i32, stat_ptr: i32) -> i32;

    #[link_name = "fd_filestat_set_size"]
    pub unsafe fn fd_filestat_set_size(fd: i32, stat: u64) -> i32;

    #[link_name = "fd_tell"]
    pub unsafe fn fd_tell(fd: i32, pos_ptr: i32) -> i32;

    #[link_name = "fd_filestat_set_times"]
    pub unsafe fn fd_filestat_set_times(fd: i32, atim: i64, mtim: i64, fst_flags: u16) -> i32;

    #[link_name = "fd_fdstat_set_flags"]
    pub unsafe fn fd_fdstat_set_flags(fd: i32, fd_flags: u16) -> i32;

    /// Reads directory entries from a file descriptor
    #[allow(dead_code)]
    #[link_name = "fd_readdir"]
    pub unsafe fn fd_readdir(fd: i32, buf: i32, buf_len: i32, cookie: u64, readn_ptr: i32) -> i32;

}
