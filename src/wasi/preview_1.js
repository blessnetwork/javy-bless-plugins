
// Wrap everything in an anonymous function to avoid leaking local variables into the global scope.
(function () {
    
    let lastErr = {
        errno: 0,
        error: "",
    }
    globalThis.lastErr = lastErr;
    // Get a reference to the function before we delete it from `globalThis`.
    const __javy_wasi_preview1_open = globalThis.__javy_wasi_preview1_open;
    const __javy_wasi_preview1_fd_prestat_dir_name = globalThis.__javy_wasi_preview1_fd_prestat_dir_name;
    const __javy_wasi_preview1_path_create_directory = globalThis.__javy_wasi_preview1_path_create_directory;
    const __javy_wasi_preview1_path_remove_directory = globalThis.__javy_wasi_preview1_path_remove_directory;
    const __javy_wasi_preview1_path_unlink_file = globalThis.__javy_wasi_preview1_path_unlink_file;
    const __javy_wasi_preview1_path_symlink = globalThis.__javy_wasi_preview1_path_symlink;
    const __javy_wasi_preview1_path_link = globalThis.__javy_wasi_preview1_path_link;
    const __javy_wasi_preview1_path_rename = globalThis.__javy_wasi_preview1_path_rename;
    const __javy_wasi_preview1_path_filestat_get = globalThis.__javy_wasi_preview1_path_filestat_get;

    const InvalParameter = 0x1C
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
        PATH_FILESTAT_GET: 0x40000,
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

    const Advise = {
        Normal: 0x0,
        Sequential: 0x1,
        Random: 0x2,
        Willneed: 0x3,
        Dontneed: 0x4,
    }

    const Whence = {
        SeekSet: 0x0,
        SeekCur: 0x1,
        SeekEnd: 0x2,
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

    // This function is used to open a file with the specified path and flags.
    // It first checks if the path is valid and then determines the directory file descriptor (dirfd) for the path.
    // It sets the appropriate flags and rights based on the specified mode (read, write, etc.).
    function open(path, flags = "r") {
        if (path == null) {
            lastErr.errno = InvalParameter;
            lastErr.error = "Path is required";
            return null;
        }
        const dirpathObj = dirfdForPath(path);
        if (dirpathObj == null) {
            return false;
        }
        const {dirpath, dirfd} = dirpathObj;
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
        path = path.substring(dirpath.length, path.length);
        let fd_rights_inherited = fd_rights;
        let fd_flags = 0;
        const file = __javy_wasi_preview1_open(
            dirfd,
            fd_lookup_flags,
            path,
            fd_oflags,
            fd_rights,
            fd_rights_inherited,
            fd_flags,
        )
        return file;
    }

    // This function is used to create a new directory with the specified path.
    // It first checks if the path is valid and then determines the directory file descriptor (dirfd) for the path.
    function mkdir(path) {
        if (path == null) {
            lastErr.errno = InvalParameter;
            lastErr.error = "Path is required";
            return false;
        }
        const dirpathObj = dirfdForPath(path);
        if (dirpathObj == null) {
            return false;
        }
        const {dirpath, dirfd} = dirpathObj;
        path = path.substring(dirpath.length, path.length);
        let rs = __javy_wasi_preview1_path_create_directory(dirfd, path)
        if (rs != 0) {
            return false;
        }
        return true;
    }

    // This function is used to remove a directory with the specified path.
    // It first checks if the path is valid and then determines the directory file descriptor (dirfd) for the path.
    function rmdir(path) {
        if (path == null) {
            lastErr.errno = InvalParameter;
            lastErr.error = "Path is required";
            return false;
        }
        const dirpathObj = dirfdForPath(path);
        if (dirpathObj == null) {
            return false;
        }
        const {dirpath, dirfd} = dirpathObj;
        path = path.substring(dirpath.length, path.length);
        let rs = __javy_wasi_preview1_path_remove_directory(dirfd, path)
        if (rs != 0) {
            return false;
        }
        return true;
    }

    // This function is used to unlink (delete) a file with the specified path.
    // It first checks if the path is valid and then determines the directory file descriptor (dirfd) for the path.
    function unlink(path) {
        if (path == null) {
            lastErr.errno = InvalParameter;
            lastErr.error = "Path is required";
            return false;
        }
        const dirpathObj = dirfdForPath(path);
        if (dirpathObj == null) {
            return false;
        }
        const {dirpath, dirfd} = dirpathObj;
        path = path.substring(dirpath.length, path.length);
        let rs = __javy_wasi_preview1_path_unlink_file(dirfd, path)
        if (rs != 0) {
            return false;
        }
        return true;
    }
    
    // This function is used to create a symbolic link from oldpath to newpath.
    // It first checks if the newpath is valid and then determines the directory file descriptor (dirfd) for the newpath.
    function symlink(oldpath, newpath) {
        if (oldpath == null || newpath == null) {
            lastErr.errno = InvalParameter;
            lastErr.error = "Path is required";
            return false;
        }
        const dirpathObj = dirfdForPath(newpath);
        if (dirpathObj == null) {
            return false;
        }
        const {dirpath, dirfd} = dirpathObj;
        newpath = newpath.substring(dirpath.length, newpath.length);
        let rs = __javy_wasi_preview1_path_symlink(oldpath, dirfd, newpath)
        if (rs != 0) {
            return false;
        }
        return true;
    }

    // This function is used to create a hard link from oldpath to newpath.
    function link(oldpath, newpath) {
        if (oldpath == null || newpath == null) {
            lastErr.errno = InvalParameter;
            lastErr.error = "Path is required";
            return false;
        }
        const old_dirpath_rs = dirfdForPath(oldpath);
        if (old_dirpath_rs == null) {
            return false;
        }

        const new_dirpath_rs = dirfdForPath(newpath);
        if (new_dirpath_rs == null) {
            return false;
        }
        const old_dirpath = old_dirpath_rs.dirpath;
        const old_dirfd = old_dirpath_rs.dirfd;
        
        const new_dirpath = new_dirpath_rs.dirpath;
        const new_dirfd = new_dirpath_rs.dirfd;
        newpath = newpath.substring(new_dirpath.length, newpath.length);
        oldpath = oldpath.substring(old_dirpath.length, oldpath.length);
        let rs = __javy_wasi_preview1_path_link(old_dirfd, 0, oldpath, new_dirfd, newpath);
        if (rs != 0) {
            return false;
        }
        return true;
    }

    function rename(oldpath, newpath) {
        if (oldpath == null || newpath == null) {
            lastErr.errno = InvalParameter;
            lastErr.error = "Path is required";
            return false;
        }
        const old_dirpath_rs = dirfdForPath(oldpath);
        if (old_dirpath_rs == null) {
            return false;
        }

        const new_dirpath_rs = dirfdForPath(newpath);
        if (new_dirpath_rs == null) {
            return false;
        }
        const old_dirpath = old_dirpath_rs.dirpath;
        const old_dirfd = old_dirpath_rs.dirfd;
        
        const new_dirpath = new_dirpath_rs.dirpath;
        const new_dirfd = new_dirpath_rs.dirfd;
        newpath = newpath.substring(new_dirpath.length, newpath.length);
        oldpath = oldpath.substring(old_dirpath.length, oldpath.length);
        let rs = __javy_wasi_preview1_path_rename(old_dirfd, oldpath, new_dirfd, newpath);
        if (rs != 0) {
            return false;
        }
        return true;
    }

    function stat(path) {
        if (path == null) {
            lastErr.errno = InvalParameter;
            lastErr.error = "Path is required";
            return false;
        }
        const dirpath_rs = dirfdForPath(path);
        if (dirpath_rs == null) {
            return false;
        }

       
        const dirpath = dirpath_rs.dirpath;
        const dirfd = dirpath_rs.dirfd;
        
        path = path.substring(dirpath.length, path.length);
        let stat = __javy_wasi_preview1_path_filestat_get(dirfd, Lookupflags.SYMLINK_FOLLOW, path);
        return stat;
    }

    // This function is used to get the directory name for a given file descriptor.
    // It recursively calls itself with an incremented file descriptor until it finds a valid directory name.
    function dirfdForPath(path, fd = 3) {
        let rs = __javy_wasi_preview1_fd_prestat_dir_name(fd);
        if (rs.code == 0) {
            if (path.startsWith(rs.dir_name)) {
                rs.fd = fd;
                return {dirpath: rs.dir_name, dirfd: fd};
            } else {
                return dirfdForPath(path, fd + 1);
            }
        } else {
            return null;
        }
    }

    globalThis.wasi_fs = function () {
        return {
            open,
            mkdir,
            rmdir,
            unlink,
            symlink,
            link,
            rename,
            stat,
            Advise,
            Whence,
            errno: () => globalThis.lastErr.errno,
            error: () => globalThis.lastErr.error,
        };
    }();

    // Delete the function from `globalThis` so it doesn't leak.
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_open");
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_fd_prestat_dir_name");
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_path_create_directory");
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_path_remove_directory");
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_path_unlink_file");
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_path_symlink");
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_path_link");
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_path_rename");
    Reflect.deleteProperty(globalThis, "__javy_wasi_preview1_path_filestat_get");
})();