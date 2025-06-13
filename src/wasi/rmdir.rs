use anyhow::{anyhow, bail, Result};
use javy_plugin_api::javy::{quickjs::Value, Args};

use super::{preview_1, process_error};

/// Remove a directory at the given path.
/// This function is used to remove a directory at the given path.
/// It is used to remove a directory at the given path.
/// - `old_dirfd`: The directory file descriptor of the old directory.
/// - `path`: The path of the directory to remove.
pub fn wasi_preview1_path_remove_directory(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>] = &args.0;
    let [dirfd, path, ..] = args_pat else {
        bail!(
            "path_remove_directory expects 2 parameters: the dirfd and path, Got: {} parameters.",
            args.len()
        );
    };
    let dirfd = dirfd
        .as_int()
        .ok_or_else(|| anyhow!("dirfd must be a number"))?;
    let path = path
        .as_string()
        .ok_or_else(|| anyhow!("path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in path"))?;
    let path_ptr = path.as_ptr() as i32;
    let path_len = path.len() as i32;
    let rs = unsafe { preview_1::path_remove_directory(dirfd, path_ptr, path_len) };
    process_error(cx.clone(), rs)?;
    Ok(Value::new_int(cx.clone(), rs))
}
