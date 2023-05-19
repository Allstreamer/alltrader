macro_rules! UnwrapReq {
    ($req:expr, $id:expr) => {
        match $req {
            Ok(v) => Some((v, $id)),
            Err(e) => {
                dbg!(&e);
                None
            }
        }
    };
}

/// This macro should only ever consume
/// results from try_locks in the following format:
/// Result<MutexGuard<'_, T>, TryLockError>
macro_rules! ContinueLock {
    ($lock_get:expr) => {
        match $lock_get {
            Ok(v) => v,
            _ => return,
        }
    };
}

macro_rules! Here {
    () => {
        format!("{} - {}:{}", file!(), line!(), column!())
    };
}

pub(crate) use ContinueLock;
pub(crate) use Here;
pub(crate) use UnwrapReq;
