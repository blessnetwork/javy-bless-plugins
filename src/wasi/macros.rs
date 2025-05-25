
macro_rules! jsvalue2int64 {
    ($i: ident) => {
        if $i.is_int() {
            $i.as_int().ok_or_else(|| anyhow!("{} must be a int", stringify!($i)))? as _
        } else {
            $i.as_big_int()
                .map(|o| o.clone())
                .ok_or_else(|| anyhow!("{} must be a int", stringify!($i)))?
                .to_i64()? as _
        };
    };
}