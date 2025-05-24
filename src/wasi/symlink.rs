use javy_plugin_api::javy::{
    quickjs::{Object as JObject, Value}, 
    Args
};
use anyhow::{anyhow, bail, Result};

use super::{preview_1, process_error};

/// This function creates a symbolic link to a file or directory.
/// It takes the following parameters:
/// - `old_path`: The path to the file or directory to be linked.
/// - `dirfd`: The file descriptor of the directory where the link will be created.
/// - `new_path`: The name of the new symbolic link.
pub fn wasi_preview1_path_symlink(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>]= &args.0;
    let [
        old_path,
        dirfd,
        new_path,
        ..
    ] =  args_pat else {
        bail!(
            "path_symlink expects 3 parameters: the old_path, fd and new_path, Got: {} parameters.",
            args.len()
        );
    };
    let old_path = old_path.as_string()
        .ok_or_else(|| anyhow!("old_path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in old_path"))?;
    let dirfd = dirfd.as_int()
        .ok_or_else(|| anyhow!("fd must be a number"))?;
    let new_path = new_path.as_string()
        .ok_or_else(|| anyhow!("new_path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in new_path"))?;
    let old_path_ptr = old_path.as_ptr() as i32;
    let old_path_len = old_path.len() as i32;
    let new_path_ptr = new_path.as_ptr() as i32;
    let new_path_len = new_path.len() as i32;
    let rs = unsafe { preview_1::path_symlink(
        old_path_ptr, old_path_len, 
        dirfd, 
        new_path_ptr, new_path_len
    )};
    process_error(cx.clone(), rs)?;
    Ok(Value::new_null(cx.clone()))
}