macro_rules! UnwrapReq {
    ($req:expr, $id:expr) => {
        match $req {
            Ok(v) => Some((v, $id)),
            Err(e) => {
                dbg!(e);
                None
            }
        }
    };
}

macro_rules! ExpectLock {
    ($lock_get:expr) => {
        $lock_get.expect("Tried to aquire lock on Mutex that was owned by panicked thread!")
    };
}
pub(crate) use UnwrapReq;
pub(crate) use ExpectLock;