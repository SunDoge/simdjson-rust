macro_rules! impl_drop {
    ($name:ident < $($lt:lifetime),+ > , $free_fn:expr) => {
        impl<$($lt),+> Drop for $name<$($lt),+> {
            fn drop(&mut self) {
                unsafe {
                    $free_fn(self.ptr.as_ptr());
                }
            }
        }
    };
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

macro_rules! map_result {
    (primitive, $func_call:expr, $get_code:expr, $get_value:expr) => {
        unsafe {
            let ptr = $func_call;
            let code = $get_code(ptr);
            if code == 0 {
                Ok($get_value(ptr))
            } else {
                Err(crate::error::SimdJsonError::from(code))
            }
        }
    };
    ($func_call:expr, $get_code:expr, $get_value:expr) => {
        unsafe {
            let ptr = $func_call;
            let code = $get_code(ptr);
            if code == 0 {
                Ok(std::ptr::NonNull::new_unchecked($get_value(ptr)))
            } else {
                Err(crate::error::SimdJsonError::from(code))
            }
        }
    };
}

pub(crate) use impl_drop;
pub(crate) use map_result;
