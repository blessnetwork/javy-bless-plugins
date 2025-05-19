
// Wrap everything in an anonymous function to avoid leaking local variables into the global scope.
(function () {
    // Get a reference to the function before we delete it from `globalThis`.
    const __javy_wasi_preview1_open = globalThis.__javy_wasi_preview1_open;
    const __javy_wasi_preview1_fd_prestat_dir_name = globalThis.__javy_wasi_preview1_fd_prestat_dir_name;
    const Rights  = {
        FD_DATASYNC: 0x1,
        FD_READ: 0x2,
        FD_SEEK: 0x4,
        FD_FDSTAT_SET_FLAGS: 0x8,
        FD_SYNC: 0x10,
        FD_TELL: 0x20,
        FD_WRITE: 0x40,
        FD_ADVISE: 0x80,
        FD_ALLOCATE: 0x100,
        PATH_CREATE_DIRECTORY: 0x200,
        PATH_CREATE_FILE: 0x400 ,
        PATH_LINK_SOURCE: 0x800,
        PATH_LINK_TARGET: 0x1000,
        PATH_OPEN: 0x2000,
        FD_READDIR: 0x4000,
        PATH_READLINK: 0x8000,
        PATH_RENAME_SOURCE: 0x10000,
        PATH_RENAME_TARGET: 0x20000,
        PATH_FILESTAT_GET: 40000,
        PATH_FILESTAT_SET_SIZE: 0x80000,
        PATH_FILESTAT_SET_TIMES: 0x100000,
        FD_FILESTAT_GET: 0x200000,
        FD_FILESTAT_SET_SIZE: 0x400000,
        FD_FILESTAT_SET_TIMES: 0x800000,
        PATH_SYMLINK: 0x1000000,
        PATH_REMOVE_DIRECTORY: 0x2000000,
        PATH_UNLINK_FILE: 0x4000000,
        POLL_FD_READWRITE: 0x8000000,
        SOCK_SHUTDOWN: 0x10000000,
        SOCK_ACCEPT: 0x20000000,
    }

    const Lookupflags = {
        SYMLINK_FOLLOW: 0x1,
    }

    const Oflags = {
        CREAT: 0x1,
        DIRECTORY: 0x2,
        EXCL: 0x4,
        TRUNC: 0x8,
    }

    function open(path, flags = "r") {
        if (path == null) {
            throw new Error("Open error: Path is required");
        }
        let dirfd_rs = dirfdForPath(path);
        let dirfd = dirfd_rs.fd;
        let fd_lookup_flags = Lookupflags.SYMLINK_FOLLOW;;
        let fd_oflags = 0;
        let fd_rights = 0;
        if (flags == "r") {
            fd_rights =
            Rights.FD_READ | Rights.FD_SEEK | Rights.FD_TELL | Rights.FD_FILESTAT_GET |
            Rights.FD_READDIR;
        } else if (flags == "r+") {
            fd_rights =
            Rights.FD_WRITE |
            Rights.FD_READ | Rights.FD_SEEK | Rights.FD_TELL | Rights.FD_FILESTAT_GET |
            Rights.PATH_CREATE_FILE;
        } else if (flags == "w") {
            fd_oflags = Oflags.CREAT | Oflags.TRUNC;
            fd_rights =
            Rights.FD_WRITE | Rights.FD_SEEK | Rights.FD_TELL | Rights.FD_FILESTAT_GET |
            Rights.PATH_CREATE_FILE;
        } else if (flags == "wx") {
            fd_oflags = Oflags.CREAT | Oflags.TRUNC | Oflags.EXCL;
            fd_rights =
            Rights.FD_WRITE | Rights.FD_SEEK | Rights.FD_TELL | Rights.FD_FILESTAT_GET |
            Rights.PATH_CREATE_FILE;
        } else if (flags == "w+") {
            fd_oflags = Oflags.CREAT | Oflags.TRUNC;
            fd_rights =
            Rights.FD_WRITE |
            Rights.FD_READ | Rights.FD_SEEK | Rights.FD_TELL | Rights.FD_FILESTAT_GET |
            Rights.PATH_CREATE_FILE;
        } else if (flags == "xw+") {
            fd_oflags = Oflags.CREAT | Oflags.TRUNC | Oflags.EXCL;
            fd_rights =
            Rights.FD_WRITE |
            Rights.FD_READ | Rights.FD_SEEK | Rights.FD_TELL | Rights.FD_FILESTAT_GET |
            Rights.PATH_CREATE_FILE;
        } else {
            return null;
        }
        path = path.substring(dir_name_rs.dir_name.length, path.length);
        let fd_rights_inherited = fd_rights;
        let fd_flags = 0;
        let rs = __javy_wasi_preview1_open(
            dirfd,
            fd_lookup_flags,
            path,
            fd_oflags,
            fd_rights,
            fd_rights_inherited,
            fd_flags,
        )
        if (rs < 0) {
            throw new Error("Open error: " + rs);
        }
        return rs;
    }

    function dirfdForPath(path, fd = 3) {
        let dir_name_rs = __javy_wasi_preview1_fd_prestat_dir_name(fd);
        if (dir_name_rs.code == 0) {
            if (path.startsWith(dir_name_rs.dir_name)) {
                dir_name_rs.fd = fd;
                return dir_name_rs;
            } else {
                return dirfdForPath(path, fd + 1);
            }
        } else {
            throw new Error("wasi_preview1_fd_prestat_dir_name error: " + dir_name_rs.code);
        }
        return null;
    }

    globalThis.wasi_preview1 = function () {
        return {
            open,
        };
    }();

    // Delete the function from `globalThis` so it doesn't leak.
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_open");

    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_fd_prestat_dir_name");
})();