macro_rules! impl_drop {
    ($name:ty, $free_fn:expr) => {
        impl Drop for $name {
            fn drop(&mut self) {
                unsafe {
                    $free_fn(self.ptr.as_ptr());
                }
            }
        }
    };
}

pub(crate) use impl_drop;
