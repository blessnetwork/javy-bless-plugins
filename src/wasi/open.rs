use anyhow::{anyhow, bail, Result};
use javy_plugin_api::javy::{quickjs::Value, Args};

use super::descriptor::Descriptor;
use super::{preview_1, process_error};

/// This function is used to open a file at the given path.
/// It is used to open a file at the given path.
pub fn wasi_preview1_open<'a>(args: Args<'a>) -> Result<Value<'a>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>] = &args.0;
    let mut opened_fd: i32 = 0;
    let [dirfd, fd_lookup_flags, path, fd_oflags, fd_rights, fd_rights_inherited, fd_flags, ..] =
        args_pat
    else {
        bail!(
            "open expects 7 parameters: the path and the dirfd, fd_lookup_flags, path, fd_oflags, fd_rights ...  Got: {} parameters.",
            args.len()
        );
    };
    let dirfd = dirfd
        .as_int()
        .ok_or_else(|| anyhow!("dirfd must be a number"))?;
    let fd_lookup_flags = fd_lookup_flags
        .as_int()
        .ok_or_else(|| anyhow!("fd_lookup_flags must be a number"))?;
    let oflags = fd_oflags
        .as_int()
        .ok_or_else(|| anyhow!("oflags must be a number"))?;
    let fs_rights_base = jsvalue2int64!(fd_rights);
    let fd_rights_inherited = jsvalue2int64!(fd_rights_inherited);
    let fdflags = fd_flags
        .as_int()
        .ok_or_else(|| anyhow!("fdflags must be a number"))?;
    let opened_fd_ptr = (&mut opened_fd as *mut i32) as i32;
    let path = path
        .as_string()
        .ok_or_else(|| anyhow!("path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in path"))?;
    let path_ptr = path.as_ptr() as i32;
    let path_len = path.len() as i32;
    let rs = unsafe {
        preview_1::path_open(
            dirfd,
            fd_lookup_flags,
            path_ptr,
            path_len,
            oflags,
            fs_rights_base,
            fd_rights_inherited,
            fdflags,
            opened_fd_ptr,
        )
    };
    if rs == 0 {
        Ok(Descriptor::new(cx.clone(), opened_fd)?)
    } else {
        process_error(cx.clone(), rs)?;
        Ok(Value::new_null(cx.clone()))
    }
}
