use error::WasiError;
use javy_plugin_api::javy::{
    quickjs::{Object as JObject, Value}, 
    Args
};
use anyhow::{anyhow, bail, Result};

mod preview_1;
mod error;

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

#[inline]
fn set_error(obj: &JObject, rs: i32) -> Result<()> {
    let error_messgae = if rs != 0 {
        let error: WasiError = rs.into();
        error.to_string()
    } else {
        "Success".to_string()
    };
    obj.set("errno", rs)?;
    obj.set("error", error_messgae)?;
    Ok(())
}

/// This function is used to close a file descriptor.
pub fn wasi_preview1_close<'a>(args: Args<'a>) -> Result<Value<'a>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>]= &args.0;
    let [
        fd,
        ..
    ] =  args_pat else {
        bail!(
            "close expects 1 parameter: the fd, Got: {} parameters.",
            args.len()
        );
    };
    let fd = fd.as_int()
        .ok_or_else(|| anyhow!("fd must be a number"))?;
    let rs = unsafe { preview_1::fd_close(fd) };
    let rs_obj = JObject::new(cx.clone())?;
    set_error(&rs_obj, rs)?;
    Ok(Value::from_object(rs_obj))
}

/// This function is used to create a directory at the given path.
/// It is used to create a directory at the given path.
/// The directory must not exist.
pub fn wasi_preview1_path_create_directory(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>]= &args.0;
    let [
        dirfd,
        path,
        ..
    ] =  args_pat else {
        bail!(
            "path_create_directory expects 2 parameters: the dirfd and path, Got: {} parameters.",
            args.len()
        );
    };
    let dirfd = dirfd.as_int()
        .ok_or_else(|| anyhow!("dirfd must be a number"))?;
    let path = path.as_string()
        .ok_or_else(|| anyhow!("path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in path"))?;
    let path_ptr = path.as_ptr() as i32;
    let path_len = path.len() as i32;
    let rs = unsafe { preview_1::path_create_directory(dirfd, path_ptr, path_len) };
    let rs_obj = JObject::new(cx.clone())?;
    set_error(&rs_obj, rs)?;
    Ok(Value::from_object(rs_obj))
}

/// Remove a directory at the given path.
/// This function is used to remove a directory at the given path.
/// It is used to remove a directory at the given path.
pub fn wasi_preview1_path_remove_directory(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>]= &args.0;
    let [
        dirfd,
        path,
        ..
    ] =  args_pat else {
        bail!(
            "path_remove_directory expects 2 parameters: the dirfd and path, Got: {} parameters.",
            args.len()
        );
    };
    let dirfd = dirfd.as_int()
        .ok_or_else(|| anyhow!("dirfd must be a number"))?;
    let path = path.as_string()
        .ok_or_else(|| anyhow!("path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in path"))?;
    let path_ptr = path.as_ptr() as i32;
    let path_len = path.len() as i32;
    let rs = unsafe { preview_1::path_remove_directory(dirfd, path_ptr, path_len) };
    let rs_obj = JObject::new(cx.clone())?;
    set_error(&rs_obj, rs)?;
    Ok(Value::from_object(rs_obj))
}

/// Unlink a file at the given path.
pub fn wasi_preview1_path_unlink_file(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>]= &args.0;
    let [
        dirfd,
        path,
        ..
    ] =  args_pat else {
        bail!(
            "path_unlink_file expects 2 parameters: the dirfd and path, Got: {} parameters.",
            args.len()
        );
    };

    // dirfd is the file descriptor of the directory
    let dirfd = dirfd.as_int()
        .ok_or_else(|| anyhow!("dirfd must be a number"))?;
    // path is the path to the file
    let path = path.as_string()
        .ok_or_else(|| anyhow!("path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in path"))?;
    let path_ptr = path.as_ptr() as i32;
    let path_len = path.len() as i32;
    let rs = unsafe { preview_1::path_unlink_file(dirfd, path_ptr, path_len) };
    let rs_obj = JObject::new(cx.clone())?;
    set_error(&rs_obj, rs)?;
    Ok(Value::from_object(rs_obj))
}


/// This function is used to get the directory name of a file descriptor.
/// It is used to get the directory name of a file descriptor.
/// The file descriptor must be a directory.
pub fn wasi_preview1_fd_prestat_dir_name(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>]= &args.0;
    let [
        fd,
        ..
    ] =  args_pat else {
        bail!(
            "fd_prestat_dir_name expects 1 parameters: the fd, path_ptr and path_len, Got: {} parameters.",
            args.len()
        );
    };
    let mut path_len_buf = [0u8; 8];
    let fd = fd.as_int()
        .ok_or_else(|| anyhow!("fd must be a number"))?;
    let path_len_ptr: i32 = path_len_buf.as_mut_ptr() as i32;
    let rs = unsafe { preview_1::fd_prestat_get(fd, path_len_ptr) };
    let path_len_buf: [u8; 4] = path_len_buf[4..].try_into()?;
    let path_len = i32::from_le_bytes(path_len_buf);
    let obj = JObject::new(cx)?;
    if rs != 0  {
        set_error(&obj, rs)?;
        return Ok(Value::from_object(obj))
    }
    let mut path_buf = vec![0u8; path_len as usize];
    let rs = unsafe { 
        preview_1::fd_prestat_dir_name(
            fd, 
            path_buf.as_mut_ptr() as *const i32 as i32, 
            path_len as _
        ) 
    };
    if rs == 0  {
        let path = String::from_utf8(path_buf)?;
        obj.set("dir_name", path)?;
    }
    set_error(&obj, rs)?;
    Ok(Value::from_object(obj))
}



