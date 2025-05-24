use std::sync::Arc;
use javy_plugin_api::javy::{
    quickjs::{
        prelude::{MutFn, Rest}, 
        Ctx, 
        Function, 
        Object as JObject, 
        TypedArray, 
        Value
    }, to_js_error
};
use anyhow::{anyhow, bail, Ok, Result};

use super::{preview_1, process_error};

pub struct Descriptor {
    fd: i32,
    errno: i32,
    error: Option<String>,
}

pub struct Iovec {
    pub buf: i32,
    pub buf_len: u32,
}

impl Descriptor {
    pub fn new<'js>(cx: Ctx<'js>, fd: i32) -> Result<Value<'js>> {
        let descriptor = Arc::new(Descriptor {
            fd,
            errno: 0,
            error: None,
        });
        let desc = JObject::new(cx.clone())?;
        desc.set("fd", fd)?;
        desc.set("read", Function::new(
            cx.clone(),
            MutFn::new(move |cx: Ctx<'js>, args: Rest<Value<'js>>| {
                descriptor.clone().read(cx.clone(), args)
                    .map_err(|e| to_js_error(cx.clone(), e))
            }),
        )?)?;
        Ok(Value::from_object(desc))
    }

    fn read<'js>(self: Arc<Self>, cx: Ctx<'js>, args: Rest<Value<'js>>) -> Result<Value<'js>> {
        let args_pat: &[Value<'_>]= &args.0;
        let [
            buffer,
            size,
            ..
        ] =  args_pat else {
            bail!(
                "read expects 2 parameters: the buffer and size, Got: {} parameters.",
                args.len()
            );
        };
        let mut readn: i32 = 0;
        let readn_ptr: i32 = &mut readn as *mut i32 as i32;
        let array: &TypedArray<'_, u8> = buffer.as_array()
            .ok_or_else(|| anyhow!("buffer must be a array"))?
            .as_typed_array()
            .ok_or_else(|| anyhow!("buffer must be a Uint8Array"))?;
        let mut array_raw = array.as_raw()
            .ok_or_else(|| anyhow!("buffer get raw ptr error."))?;
        let ioslice = vec![
            Iovec {
                buf: unsafe {array_raw.ptr.as_mut() as *mut u8  as i32},
                buf_len: size.as_int().ok_or_else(|| anyhow!("size must be a int"))? as u32,
            }
        ];
        let rs = unsafe {
            preview_1::fd_read(
                self.fd,
                ioslice.as_ptr() as i32,
                ioslice.len() as i32,
                readn_ptr)
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, readn))
    }
}

