use anyhow::{anyhow, bail, Result};
use javy_plugin_api::javy::{
    quickjs::{Ctx, Object as JObject, Value},
    Args,
};

use super::{preview_1, process_error, FileType, Filestat};

/// Get the file status of a file at the given path.
/// This function is used to get the file status of a file at the given path.
/// - `dirfd`: The directory file descriptor of the file.
/// - `lookup_flags`: Flags for looking up the file descriptor.
/// - `path`: The path of the file to get the status of.
pub fn wasi_preview1_path_filestat_get(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let args_pat: &[Value<'_>] = &args.0;
    let [dirfd, lookup_flags, path, ..] = args_pat else {
        bail!(
            "wasi_preview1_path_filestat_get expects 3 parameters: the dirfd and path, Got: {} parameters.",
            args.len()
        );
    };
    let dirfd = dirfd
        .as_int()
        .ok_or_else(|| anyhow!("dirfd must be a number"))?;
    let lookup_flags = lookup_flags
        .as_int()
        .ok_or_else(|| anyhow!("lookup_flags must be a number"))?;
    let path = path
        .as_string()
        .ok_or_else(|| anyhow!("path must be a string"))?
        .to_string()
        .map_err(|_| anyhow!("invalid UTF-8 in path"))?;
    let path_ptr = path.as_ptr() as i32;
    let path_len = path.len() as i32;
    let mut fd_stat: Filestat = Default::default();
    let fd_stat_ptr = &mut fd_stat as *mut _ as i32;
    let rs = unsafe {
        preview_1::path_filestat_get(dirfd, lookup_flags, path_ptr, path_len, fd_stat_ptr)
    };
    if rs == 0 {
        let stat = filestate_to_jsobject(cx.clone(), &fd_stat)?;
        Ok(Value::from_object(stat))
    } else {
        process_error(cx.clone(), rs)?;
        Ok(Value::new_null(cx.clone()))
    }
}

pub fn filestate_to_jsobject<'js>(cx: Ctx<'js>, fd_stat: &Filestat) -> Result<JObject<'js>> {
    let stat = JObject::new(cx.clone())?;
    stat.set("filetype", fd_stat.filetype)?;
    let filetype: &str = FileType(fd_stat.filetype).into();
    stat.set("filetype_desc", filetype)?;
    stat.set("filesize", fd_stat.size)?;
    stat.set("access_time", fd_stat.atim)?;
    stat.set("modification_time", fd_stat.mtim)?;
    stat.set("creation_time", fd_stat.ctim)?;
    Ok(stat)
}
