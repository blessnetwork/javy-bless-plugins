
use javy_plugin_api::javy::{
    quickjs::{Object as JObject, Value}, 
    Args
};
use anyhow::{anyhow, bail, Result};

mod preview_1;
mod error;
mod open;
mod mkdir;
mod rmdir;
mod unlink;
mod close;
mod symlink;
mod link;
pub(crate) use open::wasi_preview1_open;
pub(crate) use mkdir::wasi_preview1_path_create_directory;
pub(crate) use rmdir::wasi_preview1_path_remove_directory;
pub(crate) use unlink::wasi_preview1_path_unlink_file;
pub(crate) use close::wasi_preview1_close;
pub(crate) use symlink::wasi_preview1_path_symlink;
pub(crate) use link::wasi_preview1_path_link;
pub use error::WasiError;


#[inline]
pub fn process_error(obj: &JObject, rs: i32) -> Result<()> {
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
        process_error(&obj, rs)?;
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
    process_error(&obj, rs)?;
    Ok(Value::from_object(obj))
}



