use javy_plugin_api::javy::{
    quickjs::{Object as JObject, Value}, 
    Args
};
use anyhow::{anyhow, bail, Result};

use super::{preview_1, set_error};

/// This function is used to open a file at the given path.
/// It is used to open a file at the given path.
pub fn wasi_preview1_open<'a>(args: Args<'a>) -> Result<Value<'a>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>]= &args.0;
    let mut opened_fd: i32 = 0;
    let [
        dirfd, 
        fd_lookup_flags,
        path, 
        fd_oflags, 
        fd_rights, 
        fd_rights_inherited, 
        fd_flags,
        ..
    ] =  args_pat else {
        bail!(
            "open expects 7 parameters: the path and the dirfd, fd_lookup_flags, path, fd_oflags, fd_rights ...  Got: {} parameters.",
            args.len()
        );
    };
    let dirfd = dirfd.as_int()
        .ok_or_else(|| anyhow!("dirfd must be a number"))?;
    let fd_lookup_flags = fd_lookup_flags.as_int()
        .ok_or_else(|| anyhow!("fd_lookup_flags must be a number"))?;
    let oflags = fd_oflags.as_int()
        .ok_or_else(|| anyhow!("oflags must be a number"))?;
    let fs_rights_base = if fd_rights.is_int() {
        fd_rights.as_int()
            .map(|i| i as i64)
            .ok_or_else(|| anyhow!("fd_rights must be a number"))?
    } else {
        fd_rights.as_big_int()
            .map(|x| x.clone())
            .ok_or_else(|| anyhow!("fd_rights must be a number"))?
            .to_i64()?
    };
    let fd_rights_inherited = if fd_rights_inherited.is_int() {
        fd_rights_inherited.as_int()
            .map(|i| i as i64)
            .ok_or_else(|| anyhow!("fd_rights_inherited must be a number"))?
    } else {
        fd_rights_inherited.as_big_int()
            .map(|x| x.clone())
            .ok_or_else(|| anyhow!("fd_rights_inherited must be a number"))?
            .to_i64()?
    };
    let fdflags = fd_flags.as_int()
        .ok_or_else(|| anyhow!("fdflags must be a number"))?;
    let opened_fd_ptr = (&mut opened_fd as *mut i32) as i32;
    let path = path.as_string()
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
        opened_fd_ptr)
    };

    let rs_obj = JObject::new(cx.clone())?;
    rs_obj.set("fd", opened_fd)?;
    set_error(&rs_obj, rs)?;
    Ok(Value::from_object(rs_obj))
}
