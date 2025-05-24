use std::sync::Arc;
use javy_plugin_api::javy::{
    quickjs::{
        prelude::{MutFn, Rest}, Ctx, Function, Object as JObject, TypedArray, Value
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
        // Set the read method
        let descriptor_clone = descriptor.clone();
        desc.set("read", Function::new(
            cx.clone(),
            MutFn::new(move |cx: Ctx<'js>, args: Rest<Value<'js>>| {
                descriptor_clone.clone().read(cx.clone(), args)
                    .map_err(|e| to_js_error(cx.clone(), e))
            }),
        )?)?;
        // Set the write method
        let descriptor_clone = descriptor.clone();
        desc.set("write", Function::new(
            cx.clone(),
            MutFn::new(move |cx: Ctx<'js>, args: Rest<Value<'js>>| {
                descriptor_clone.clone().write(cx.clone(), args)
                    .map_err(|e| to_js_error(cx.clone(), e))
            }),
        )?)?;
        // Set the close method
        let descriptor_clone = descriptor.clone();
        desc.set("close", Function::new(
            cx.clone(),
            MutFn::new(move |cx: Ctx<'js>, args: Rest<Value<'js>>| {
                descriptor_clone.clone().write(cx.clone(), args)
                    .map_err(|e| to_js_error(cx.clone(), e))
            }),
        )?)?;
        Ok(Value::from_object(desc))
    }

    /// The read method
    /// # Arguments
    fn read<'js>(self: Arc<Self>, cx: Ctx<'js>, args: Rest<Value<'js>>) -> Result<Value<'js>> {
        if args.0.len() < 1 {
            bail!(
                "read expects 1 parameters: the buffer and size[option], Got: {} parameters.",
                args.len()
            );
        }
        let buffer =  &args.0[0];
        let null = Value::new_null(cx.clone());
        let mut size = &null;
        if args.0.len() > 2 {
            size = &args.0[1];
        }

        let mut readn: i32 = 0;
        let readn_ptr: i32 = &mut readn as *mut i32 as i32;
        let array = buffer.as_object()
            .ok_or_else(|| anyhow!("buffer must be a array"))?;
        let array: &TypedArray<'_, u8> = array.as_typed_array()
            .ok_or_else(|| anyhow!("buffer must be a typed array"))?;
        let mut array_raw = array.as_raw()
            .ok_or_else(|| anyhow!("buffer get raw ptr error."))?;
        let size = size.as_int();
        let size = if let Some(size) = size {
            if size > array_raw.len as i32 {
                array_raw.len as u32
            } else {
                size as u32
            }
        } else {
            array_raw.len as u32
        };
        let ioslice = vec![
            Iovec {
                buf: unsafe {array_raw.ptr.as_mut() as *mut u8  as i32},
                buf_len: size,
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
        if rs != 0 {
            readn = -rs;
        }
        Ok(Value::new_int(cx, readn))
    }

    /// The write method
    /// Uint8Array as the buffer the first parameter
    /// size as the second parameter, it's optional, default is the length of the buffer
    fn write<'js>(self: Arc<Self>, cx: Ctx<'js>, args: Rest<Value<'js>>) -> Result<Value<'js>> {
        if args.0.len() < 1 {
            bail!(
                "write expects 1 parameters: the buffer and size[option], Got: {} parameters.",
                args.len()
            );
        }
        let buffer =  &args.0[0];
        let null = Value::new_null(cx.clone());
        let mut size = &null;
        if args.0.len() > 2 {
            size = &args.0[1];
        }

        let mut writen: i32 = 0;
        let writen_ptr: i32 = &mut writen as *mut i32 as i32;
        let array = buffer.as_object()
            .ok_or_else(|| anyhow!("buffer must be a array"))?;
        let array: &TypedArray<'_, u8> = array.as_typed_array()
            .ok_or_else(|| anyhow!("buffer must be a typed array"))?;
        let mut array_raw = array.as_raw()
            .ok_or_else(|| anyhow!("buffer get raw ptr error."))?;
        let size = size.as_int();
        let size = if let Some(size) = size {
            if size > array_raw.len as i32 {
                array_raw.len as u32
            } else {
                size as u32
            }
        } else {
            array_raw.len as u32
        };
        let ioslice = vec![
            Iovec {
                buf: unsafe {array_raw.ptr.as_mut() as *mut u8  as i32},
                buf_len: size,
            }
        ];
        let rs = unsafe {
            preview_1::fd_write(
                self.fd,
                ioslice.as_ptr() as i32,
                ioslice.len() as i32,
                writen_ptr)
        };
        process_error(cx.clone(), rs)?;
        if rs != 0 {
            writen = -rs;
        }
        Ok(Value::new_int(cx, writen))
    }

    /// The close method
    /// Uint8Array as the buffer the first parameter
    /// size as the second parameter, it's optional, default is the length of the buffer
    fn close<'js>(self: Arc<Self>, cx: Ctx<'js>, _: Rest<Value<'js>>) -> Result<Value<'js>> {
        let rs = unsafe {
            preview_1::fd_close(
                self.fd
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }
}

