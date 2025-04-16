use base64::{engine::general_purpose, Engine as _};
use javy_plugin_api::javy::{
    quickjs::{qjs::JS_GetArrayBuffer, Value},
    Args,
};
use std::slice;

/// `btoa(buffer: ArrayBuffer, offset: usize, length: usize) -> String`
pub fn bless_b64_encode(args: Args) -> anyhow::Result<Value> {
    let ctx = args.context();
    let js_val = args
        .get(0)
        .ok_or_else(|| anyhow::anyhow!("Missing buffer"))?;
    let offset = args.get(1).and_then(|v| v.as_i32()).unwrap_or(0) as usize;
    let len = args.get(2).and_then(|v| v.as_i32()).unwrap_or(0) as usize;

    unsafe {
        let mut buf_len = 0;
        let raw_ptr = JS_GetArrayBuffer(ctx.as_ptr(), &mut buf_len as *mut usize, js_val.as_ptr());
        if raw_ptr.is_null() {
            anyhow::bail!("Invalid ArrayBuffer");
        }

        let slice = slice::from_raw_parts(raw_ptr.add(offset), len);
        let encoded = general_purpose::STANDARD.encode(slice);
        Ok(Value::from_str(ctx, &encoded)?)
    }
}

/// `atob(string: base64) -> ArrayBuffer`
pub fn bless_b64_decode(args: Args) -> anyhow::Result<Value> {
    let ctx = args.context();
    let encoded = args
        .get(0)
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow::anyhow!("Expected a string"))?;

    let decoded = general_purpose::STANDARD
        .decode(encoded)
        .map_err(|e| anyhow::anyhow!("Base64 decode error: {}", e))?;

    let buf = ctx
        .array_buffer_store_bytes(&decoded)
        .map_err(|e| anyhow::anyhow!("Failed to store decoded bytes: {}", e))?;
    Ok(buf)
}
