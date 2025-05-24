use thiserror::Error;

#[derive(Error, Debug)]
pub enum WasiError {
    #[error("Argument list too long.")]
    TooBig = 1,
    #[error("Permission denied.")]
    Access,
    #[error("Address not available.")]
    AddrNotUse,
    #[error("Address not available.")]
    AddrNotAvail,
    #[error("Address family not supported..")]
    AfNoSupport,
    #[error("Resource unavailable, or operation would block.")]
    Again,
    #[error("Connection already in progress.")]
    Already,
    #[error("Bad file descriptor.")]
    Badf,
    #[error("Bad message.")]
    Badmsg,
    #[error("Device or resource busy.")]
    Busy,
    #[error("Operation canceled.")]
    Canceled,
    #[error("No child processes.")]
    Child,
    #[error("Connection aborted.")]
    Connaborted,
    #[error("Connection refused.")]
    ConnRefused,
    #[error("Connection reset.")]
    ConnReset,
    #[error("Resource deadlock would occur.")]
    Deadlk,
    #[error("Destination address required.")]
    Destaddrreq,
    #[error("Mathematics argument out of domain of function.")]
    Dom,
    #[error("Reserved.")]
    Dquot,
    #[error("File exists.")]
    Exist,
    #[error("Bad address.")]
    Fault,
    #[error("File too large.")]
    Fbig,
    #[error("Host is unreachable.")]
    Hostunreach,
    #[error("Identifier removed.")]
    Idrm,
    #[error("Illegal byte sequence.")]
    Ilseq,
    #[error("Operation in progress.")]
    Inprogress,
    #[error("Interrupted function.")]
    Intr,
    #[error("Invalid argument.")]
    Inval,
    #[error("I/O error.")]
    Io,
    #[error("Socket is connected.")]
    Isconn,
    #[error("Is a directory.")]
    Isdir,
    #[error("Too many levels of symbolic links.")]
    Loop,
    #[error("File descriptor value too large.")]
    Mfile,
    #[error("Too many links.")]
    Mlink,
    #[error("Message too large.")]
    Msgsize,
    #[error("Reserved.")]
    Multihop,
    #[error("Filename too long.")]
    Nametoolong,
    #[error("Network is down.")]
    Netdown,
    #[error("Connection aborted by network.")]
    Netreset,
    #[error("Network unreachable.")]
    Netunreach,
    #[error("Too many files open in system.")]
    Nfile,
    #[error("No buffer space available.")]
    Nobufs,
    #[error("No such device.")]
    Nodev,
    #[error("No such file or directory.")]
    Noent,
    #[error("Executable file format error.")]
    Noexec,
    #[error("No locks available.")]
    Nolck,
    #[error("Reserved.")]
    Nolink,
    #[error("Not enough space.")]
    Nomem,
    #[error("No message of the desired type.")]
    Nomsg,
    #[error("Protocol not available.")]
    Noprotoopt,
    #[error("No space left on device.")]
    Nospc,
    #[error("Function not supported.")]
    Nosys,
    #[error("The socket is not connected.")]
    Notconn,
    #[error("Not a directory or a symbolic link to a directory.")]
    Notdir,
    #[error("Directory not empty.")]
    Notempty,
    #[error("State not recoverable.")]
    Notrecoverable,
    #[error("Not a socket.")]
    Notsock,
    #[error("Not supported, or operation not supported on socket.")]
    Notsup,
    #[error("Inappropriate I/O control operation.")]
    Notty,
    #[error("No such device or address.")]
    Nxio,
    #[error("Value too large to be stored in data type.")]
    Overflow,
    #[error("Previous owner died.")]
    Ownerdead,
    #[error("Operation not permitted.")]
    Perm,
    #[error("Broken pipe.")]
    Pipe,
    #[error("Protocol error.")]
    Proto,
    #[error("Protocol not supported.")]
    Protonosupport,
    #[error("Protocol wrong type for socket.")]
    Prototype,
    #[error("Result too large.")]
    Range,
    #[error("Read-only file system.")]
    Rofs,
    #[error("Invalid seek.")]
    Spipe,
    #[error("No such process.")]
    Srch,
    #[error("Reserved.")]
    Stale,
    #[error("Connection timed out.")]
    Timedout,
    #[error("Text file busy.")]
    Txtbsy,
    #[error("Cross-device link.")]
    Xdev,
    #[error("Extension: Capabilities insufficient.")]
    Notcapable,
}

impl From<i32> for WasiError {
    fn from(code: i32) -> Self {
        match code {
            1 => WasiError::TooBig,
            2 => WasiError::Access,
            3 => WasiError::AddrNotUse,
            4 => WasiError::AddrNotAvail,
            5 => WasiError::AfNoSupport,
            6 => WasiError::Again,
            7 => WasiError::Already,
            8 => WasiError::Badf,
            9 => WasiError::Badmsg,
            10 => WasiError::Busy,
            11 => WasiError::Canceled,
            12 => WasiError::Child,
            13 => WasiError::Connaborted,
            14 => WasiError::ConnRefused,
            15 => WasiError::ConnReset,
            16 => WasiError::Deadlk,
            17 => WasiError::Destaddrreq,
            18 => WasiError::Dom,
            19 => WasiError::Dquot,
            20 => WasiError::Exist,
            21 => WasiError::Fault,
            22 => WasiError::Fbig,
            23 => WasiError::Hostunreach,
            24 => WasiError::Idrm,
            25 => WasiError::Ilseq,
            26 => WasiError::Inprogress,
            27 => WasiError::Intr,
            28 => WasiError::Inval,
            29 => WasiError::Io,
            30 => WasiError::Isconn,
            31 => WasiError::Isdir,
            32 => WasiError::Loop,
            33 => WasiError::Mfile,
            34 => WasiError::Mlink,
            35 => WasiError::Msgsize,
            36 => WasiError::Multihop,
            37 => WasiError::Nametoolong,
            38 => WasiError::Netdown,
            39 => WasiError::Netreset,
            40 => WasiError::Netunreach,
            41 => WasiError::Nfile,
            42 => WasiError::Nobufs,
            43 => WasiError::Nodev,
            44 => WasiError::Noent,
            45 => WasiError::Noexec,
            46 => WasiError::Nolck,
            47 => WasiError::Nolink,
            48 => WasiError::Nomem,
            49 => WasiError::Nomsg,
            50 => WasiError::Noprotoopt,
            51 => WasiError::Nospc,
            52 => WasiError::Nosys,
            53 => WasiError::Notconn,
            54 => WasiError::Notdir,
            55 => WasiError::Notempty,
            56 => WasiError::Notrecoverable,
            57 => WasiError::Notsock,
            58 => WasiError::Notsup,
            59 => WasiError::Notty,
            60 => WasiError::Nxio,
            61 => WasiError::Overflow,
            62 => WasiError::Ownerdead,
            63 => WasiError::Perm,
            64 => WasiError::Pipe,
            65 => WasiError::Proto,
            66 => WasiError::Protonosupport,
            67 => WasiError::Prototype,
            68 => WasiError::Range,
            69 => WasiError::Rofs,
            70 => WasiError::Spipe,
            71 => WasiError::Srch,
            72 => WasiError::Stale,
            73 => WasiError::Timedout,
            74 => WasiError::Txtbsy,
            75 => WasiError::Xdev,
            76 => WasiError::Notcapable,
            _ => unimplemented!("WasiError code: {}", code),
        }
    }
}
