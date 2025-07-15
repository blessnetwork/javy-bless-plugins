use anyhow::{anyhow, bail, Error, Result};
use javy_plugin_api::javy::{
    quickjs::{qjs::JS_GetArrayBuffer, Value},
    Args,
};
use rand::RngCore;

pub const CRYPTO_JS: &str = include_str!("crypto.js");

pub fn bless_get_random_values(args: Args<'_>) -> Result<Value<'_>> {
    let (cx, args) = args.release();
    let (data, offset, length) = extract_args(&args, "Javy.Crypto.getRandomValues")?;

    let offset = offset
        .as_number()
        .ok_or_else(|| anyhow!("offset must be a number"))? as usize;
    let length = length
        .as_number()
        .ok_or_else(|| anyhow!("length must be a number"))? as usize;

    // Safety: Port of previous implementation
    let data = unsafe {
        let mut len = 0;
        let ptr = JS_GetArrayBuffer(cx.as_raw().as_ptr(), &mut len, data.as_raw());
        if ptr.is_null() {
            bail!("Data must be an ArrayBuffer");
        }

        Ok::<_, Error>(std::slice::from_raw_parts_mut(ptr, len as _))
    }?;

    let data = &mut data[offset..(offset + length)];

    // Fill the buffer with random values
    rand::rngs::OsRng.fill_bytes(data);

    Ok(Value::new_undefined(cx.clone()))
}

fn extract_args<'a, 'js: 'a>(
    args: &'a [Value<'js>],
    for_func: &str,
) -> Result<(&'a Value<'js>, &'a Value<'js>, &'a Value<'js>)> {
    let [data, offset, length, ..] = args else {
        bail!(
            r#"{} expects 3 parameters: the file descriptor, the
           TypedArray buffer, the TypedArray byteOffset and the TypedArray
           byteLength.

           Got: {} parameters."#,
            for_func,
            args.len()
        );
    };

    Ok((data, offset, length))
}
