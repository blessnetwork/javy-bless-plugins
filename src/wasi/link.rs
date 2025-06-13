use anyhow::{anyhow, bail, Result};
use javy_plugin_api::javy::{quickjs::Value, Args};

use super::{preview_1, process_error};

/// This function is used to link a file at the given path to a new path.
/// It is used to create a hard link from one file to another.
/// - `old_dirfd`: The directory file descriptor of the old file.
/// - `fd_lookup_flags`: Flags for looking up the file descriptor.
/// - `old_path`: The path of the old file.
/// - `new_dirfd`: The directory file descriptor of the new file.
/// - `new_path`: The path of the new file.
pub fn wasi_preview1_path_link(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>] = &args.0;
    let [old_dirfd, fd_lookup_flags, old_path, new_dirfd, new_path, ..] = args_pat else {
        bail!(
            "path_link expects 5 parameters: old_dirfd, fd_lookup_flags, old_path, new_dirfd, new_path. Got: {} parameters.",
            args.len()
        );
    };
    let dirfd = old_dirfd
        .as_int()
        .ok_or_else(|| anyhow!("old_dirfd must be a number"))?;
    let fd_lookup_flags = fd_lookup_flags
        .as_int()
        .ok_or_else(|| anyhow!("fd_lookup_flags must be a number"))?;
    let old_path = old_path
        .as_string()
        .ok_or_else(|| anyhow!("old_path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in path"))?;
    let new_dirfd = new_dirfd
        .as_int()
        .ok_or_else(|| anyhow!("new_dirfd must be a number"))?;
    let new_path = new_path
        .as_string()
        .ok_or_else(|| anyhow!("new_path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in path"))?;
    let new_path_ptr = new_path.as_ptr() as i32;
    let new_path_len = new_path.len() as i32;
    let old_path_ptr = old_path.as_ptr() as i32;
    let old_path_len = old_path.len() as i32;
    let rs = unsafe {
        preview_1::path_link(
            dirfd,
            fd_lookup_flags,
            old_path_ptr,
            old_path_len,
            new_dirfd,
            new_path_ptr,
            new_path_len,
        )
    };
    process_error(cx.clone(), rs)?;
    Ok(Value::new_int(cx.clone(), rs))
}
