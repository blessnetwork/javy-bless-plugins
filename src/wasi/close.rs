use javy_plugin_api::javy::{
    quickjs::Value, 
    Args
};
use anyhow::{anyhow, bail, Result};

use super::{preview_1, process_error};

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
    process_error(cx.clone(), rs)?;
    Ok(Value::new_int(cx.clone(), rs))
}