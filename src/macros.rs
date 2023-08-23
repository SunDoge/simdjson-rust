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

macro_rules! check_result {
    ($func_call:expr, $get_code:expr, $get_value:expr) => {
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
}

pub(crate) use check_result;
pub(crate) use impl_drop;
