use std::sync::Arc;
use javy_plugin_api::javy::{
    quickjs::{
        prelude::{MutFn, Rest}, BigInt, Ctx, Function, Object as JObject, TypedArray, Value
    }, to_js_error
};
use anyhow::{anyhow, bail, Ok, Result};

use super::{preview_1, process_error, stat::filestate_to_jsobject, Filestat, Fstflags};


pub struct Descriptor(i32);

/// This struct is used to represent an I/O vector.
#[allow(dead_code)]
pub struct Iovec {
    pub buf: i32,
    pub buf_len: u32,
}

impl Descriptor {
    /// Create a new file descriptor object.
    /// This function creates a new file descriptor object with the given file descriptor.
    /// The file descriptor is used to perform operations on the file.
    pub fn new<'js>(cx: Ctx<'js>, fd: i32) -> Result<Value<'js>> {
        let descriptor = Arc::new(Descriptor(fd));
        let desc = JObject::new(cx.clone())?;
        desc.set("rawfd", fd)?;
        macro_rules! bind_method {
            ($name:ident) => {
                bind_method!(stringify!($name), $name);
            };
            ($name: expr, $method: ident) => {
                let descriptor_clone = descriptor.clone();
                desc.set($name, Function::new(
                    cx.clone(),
                    MutFn::new(move |cx: Ctx<'js>, args: Rest<Value<'js>>| {
                        descriptor_clone.clone().$method(cx.clone(), args)
                            .map_err(|e| to_js_error(cx.clone(), e))
                    }),
                )?)?;
            };
        }
        // Set the read method
        bind_method!(read);
        // Set the write method
        bind_method!(write);
        // Set the close method
        bind_method!(close);
        // Set the fsync method
        bind_method!(fsync);
        // Set the fdatsync method
        bind_method!(fdatasync);
        // Set the seek method
        bind_method!(seek);
        // Set the advise method
        bind_method!(advise);
        // Set the stat method
        bind_method!(stat);
        // Set the ftruncate method
        bind_method!(ftruncate);
        // Set the allocate method
        bind_method!(allocate);
        // Set the tell method
        bind_method!(tell);
        // Set the touch method
        bind_method!(touch);
        // Set the set_flags method
        bind_method!("setFlags", set_flags);
        Ok(Value::from_object(desc))
    }

    /// The read method
    /// Uint8Array as the buffer the first parameter
    /// size as the second parameter, it's optional, default is the length of the buffer
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
                self.0,
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
                self.0,
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

    /// The advise method
    /// This method is used to give advice to the file descriptor.
    /// The first parameter is the offset, the second parameter is the length,
    /// and the third parameter is the advice.
    /// The advice can be one of the following values:
    /// - `0`: Normal access.
    /// - `1`: Random access. 
    /// - `2`: Sequential access.
    /// - `3`: Will need to read the data.
    /// - `4`: Will need to write the data.
    /// The offset is the number of bytes to offset from the beginning of the file,
    /// and the length is the number of bytes to advise.
    fn advise<'js>(self: Arc<Self>, cx: Ctx<'js>, args: Rest<Value<'js>>) -> Result<Value<'js>> {
        let args_pat: &[Value<'_>]= &args.0;
        let [
            offset,
            len,
            advice,
            ..
        ] =  args_pat else {
            bail!(
                "advice expects 3 parameters: the offset, len and advice, Got: {} parameters.",
                args.len()
            );
        };
        let offset: u64 = jsvalue2int64!(offset);
        let len: u64 = jsvalue2int64!(len);
        let advice: i32 = advice.as_int().ok_or_else(|| anyhow!("advice must be a int"))?;
        let rs = unsafe {
            preview_1::fd_advise(
                self.0,
                offset,
                len,
                advice
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }

    /// The seek method
    /// This method is used to change the current position of the file descriptor.
    /// The first parameter is the offset, the second parameter is the whence.
    /// The whence can be one of the following values:
    /// - `0`: Seek from the beginning of the file.
    /// - `1`: Seek from the current position of the file.
    /// - `2`: Seek from the end of the file.
    /// The offset is the number of bytes to seek.
    fn seek<'js>(self: Arc<Self>, cx: Ctx<'js>, args: Rest<Value<'js>>) -> Result<Value<'js>> {
        let args_pat: &[Value<'_>]= &args.0;
        let [
            offset,
            whence,
            ..
        ] =  args_pat else {
            bail!(
                "advice expects 2 parameters: the offset and whence, Got: {} parameters.",
                args.len()
            );
        };
        let offset: u64 = jsvalue2int64!(offset);
        
        let whence: i32 = whence.as_int().ok_or_else(|| anyhow!("advice must be a int"))?;
        let mut fsize: i64 = 0;
        let fsize_ptr: i32 = &mut fsize as *mut i64 as i32;
        let rs = unsafe {
            preview_1::fd_seek(
                self.0,
                offset,
                whence,
                fsize_ptr
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }

    /// The close method
    /// Uint8Array as the buffer the first parameter
    /// size as the second parameter, it's optional, default is the length of the buffer
    fn close<'js>(self: Arc<Self>, cx: Ctx<'js>, _: Rest<Value<'js>>) -> Result<Value<'js>> {
        let rs = unsafe {
            preview_1::fd_close(
                self.0
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }

    /// The fsync method
    /// Wait for the data and metadata to be written
    fn fsync<'js>(self: Arc<Self>, cx: Ctx<'js>, _: Rest<Value<'js>>) -> Result<Value<'js>> {
        let rs = unsafe {
            preview_1::fd_sync(
                self.0
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }

    /// The fdatasync method
    /// Wait for the data to be written
    fn fdatasync<'js>(self: Arc<Self>, cx: Ctx<'js>, _: Rest<Value<'js>>) -> Result<Value<'js>> {
        let rs = unsafe {
            preview_1::fd_datasync(
                self.0
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }

    /// The stat method
    /// This method is used to get the file status of the file descriptor.
    /// It returns a JavaScript object with the following properties:
    /// - `filetype`: The type of the file.
    /// - `filetype_desc`: The description of the file type.
    /// - `filesize`: The size of the file in bytes.
    /// - `atime`: The last access time of the file.
    /// - `mtime`: The last modification time of the file.
    /// - `ctime`: The last change time of the file.
    fn stat<'js>(self: Arc<Self>, cx: Ctx<'js>, _: Rest<Value<'js>>) -> Result<Value<'js>> {
        let mut fd_stat: Filestat = Default::default();
        let fd_stat_ptr = &mut fd_stat as *mut _ as i32;
        let rs = unsafe {
            preview_1::fd_filestat_get(
                self.0,
                fd_stat_ptr,
            )
        };
        if rs == 0 {
            let stat = filestate_to_jsobject(cx.clone(), &fd_stat)?;
            Ok(Value::from_object(stat))
        } else {
            process_error(cx.clone(), rs)?;
            Ok(Value::new_null(cx.clone()))
        }
    }

    /// The allocate method
    /// This method is used to allocate space in the file descriptor.
    /// The first parameter is the offset, the second parameter is the length.
    /// The offset is the number of bytes to offset from the beginning of the file,
    /// and the length is the number of bytes to allocate.
    fn allocate<'js>(self: Arc<Self>, cx: Ctx<'js>, args: Rest<Value<'js>>) -> Result<Value<'js>> {
        let args_pat: &[Value<'_>]= &args.0;
        let [
            offset,
            len,
            ..
        ] =  args_pat else {
            bail!(
                "allocate expects 2 parameters: the offset and length, Got: {} parameters.",
                args.len()
            );
        };
        let offset: u64 = jsvalue2int64!(offset);
        let len: u64 = jsvalue2int64!(len);
        let rs = unsafe {
            preview_1::fd_allocate(
                self.0,
                offset,
                len
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }

    /// The ftruncate method
    /// This method is used to truncate the file descriptor to the given length.
    /// The first parameter is the length to truncate the file to.
    fn ftruncate<'js>(self: Arc<Self>, cx: Ctx<'js>, args: Rest<Value<'js>>) -> Result<Value<'js>> {
        let args_pat: &[Value<'_>]= &args.0;
        let [
            len,
            ..
        ] =  args_pat else {
            bail!(
                "ftruncate expects 1 parameters: the offset and whence, Got: {} parameters.",
                args.len()
            );
        };
        let len = jsvalue2int64!(len);
        let rs = unsafe {
            preview_1::fd_filestat_set_size(
                self.0,
                len
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }

    /// The tell method
    /// This method is used to get the current position of the file descriptor.
    ///  It returns a BigInt representing the current position in the file.
    fn tell<'js>(self: Arc<Self>, cx: Ctx<'js>, _: Rest<Value<'js>>) -> Result<Value<'js>> {
        let mut pos: u64 = 0;
        let pos_ptr: i32 = &mut pos as *mut u64 as i32;
        let rs = unsafe {
            preview_1::fd_tell(
                self.0,
                pos_ptr
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::from_big_int(BigInt::from_u64(cx, pos)?))
    }

    /// The touch method
    /// This method is used to update the access and modification times of the file descriptor.
    fn touch<'js>(self: Arc<Self>, cx: Ctx<'js>, _args: Rest<Value<'js>>) -> Result<Value<'js>> {
        let rs = unsafe {
            preview_1::fd_filestat_set_times(
                self.0,
                0,
                0,
                Fstflags::AtmNow as u16|Fstflags::MtimNow as u16
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }

    /// The fd_fdstat_set_flags method
    /// This method is used to set the flags of the file descriptor.
    fn set_flags<'js>(self: Arc<Self>, cx: Ctx<'js>, args: Rest<Value<'js>>) -> Result<Value<'js>> {
        let args_pat: &[Value<'_>]= &args.0;
        let [
            flags,
            ..
        ] =  args_pat else {
            bail!(
                "set_flags expects 1 parameters: the fd_flags, Got: {} parameters.",
                args.len()
            );
        };
        let fd_flags = flags.as_int().ok_or_else(|| anyhow!("fd_flags must be a int"))?;
        let rs = unsafe {
            preview_1::fd_fdstat_set_flags(
                self.0,
                fd_flags as u16
            )
        };
        process_error(cx.clone(), rs)?;
        Ok(Value::new_int(cx, rs))
    }

}

