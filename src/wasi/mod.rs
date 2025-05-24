
use javy_plugin_api::javy::{
    quickjs::{Ctx, Object as JObject, Value}, 
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
mod rename;
mod stat;
mod descriptor;
pub(crate) use open::wasi_preview1_open;
pub(crate) use mkdir::wasi_preview1_path_create_directory;
pub(crate) use rmdir::wasi_preview1_path_remove_directory;
pub(crate) use unlink::wasi_preview1_path_unlink_file;
pub(crate) use close::wasi_preview1_close;
pub(crate) use symlink::wasi_preview1_path_symlink;
pub(crate) use link::wasi_preview1_path_link;
pub(crate) use rename::wasi_preview1_path_rename;
pub(crate) use stat::wasi_preview1_path_filestat_get;
pub use error::WasiError;


#[inline]
pub fn process_error(ctx: Ctx<'_>, rs: i32) -> Result<()> {
    let obj = JObject::new(ctx.clone())?;
    let error_messgae = if rs != 0 {
        let error: WasiError = rs.into();
        error.to_string()
    } else {
        "Success".to_string()
    };
    obj.set("errno", rs)?;
    obj.set("error", error_messgae)?;
    ctx.globals().set("lastErr", obj)?;
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
    let obj = JObject::new(cx.clone())?;
    if rs != 0  {
        process_error(cx.clone(), rs)?;
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
    obj.set("code", rs)?;
    process_error(cx.clone(), rs)?;
    Ok(Value::from_object(obj))
}

#[derive(Default, Debug)]
#[repr(C)]
pub struct Filestat {
    pub device_id: u64,
    pub inode: u64,
    pub filetype: u8,
    pub nlink: u64,
    pub size: u64, // this is a read field, the rest are file fields
    pub atim: u64,
    pub mtim: u64,
    pub ctim: u64,
}

pub struct FileType(u8);

impl Into<&str> for FileType {
    fn into(self) -> &'static str {
        match self.0 {
            0 => "unknown",
            1 => "block device",
            2 => "character device",
            3 => "directory",
            4 => "regular file",
            5 => "socket dgram",
            6 => "socket stream",
            7 => "symbolic link",
            _ => unimplemented!("FileType not implemented"),
        }
    }
}