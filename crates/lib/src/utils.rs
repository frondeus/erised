use std::sync::Arc;
use std::sync::Weak;

pub trait ArcExt<T>: Sized {
    fn create_cyclic<F, E>(func: F) -> Result<Self, E>
    where
        F: FnOnce(&Weak<T>) -> Result<T, E>;
}

impl<T: Default> ArcExt<T> for Arc<T> {
    fn create_cyclic<F, E>(func: F) -> Result<Self, E>
    where
        F: FnOnce(&Weak<T>) -> Result<T, E>,
    {
        let mut err = None;
        let o = Arc::new_cyclic(|weak| match func(weak) {
            Ok(o) => o,
            Err(e) => {
                err = Some(e);
                Default::default()
            }
        });

        match err {
            None => Ok(o),
            Some(err) => Err(err),
        }
    }
}
