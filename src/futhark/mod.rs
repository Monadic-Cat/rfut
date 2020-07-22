pub mod sys;
use core::marker::PhantomData;
// use core::cell::UnsafeCell; // Is this necessary?

pub struct ContextConfig {
    ptr: *mut sys::ContextConfig,
}
impl ContextConfig {
    pub fn new() -> Self {
        Self {
            ptr: unsafe { sys::futhark_context_config_new() },
        }
    }
}

pub struct Context<'cfg, Tag> {
    ptr: *mut sys::Context,
    _tag: PhantomData<Tag>,
    _cfg: PhantomData<&'cfg ContextConfig>,
}
impl<'cfg, Tag> Context<'cfg, Tag> {
    pub unsafe fn new_unchecked(cfg: &'cfg ContextConfig) -> Self {
        #[allow(unused_unsafe)]
        Self {
            ptr: unsafe { sys::futhark_context_new(cfg.ptr) },
            _tag: PhantomData,
            _cfg: PhantomData,
        }
    }
    pub fn avg(&self, data: F64Array1DRef<'cfg, '_, Tag>) -> Result<f64, i32> {
        let mut out = 0f64;
        let res = unsafe { sys::futhark_entry_avg(self.ptr, &mut out, data.ptr) };
        if res != 0 { Err(res) } else { Ok(out) }
    }
}

pub struct F64Array1DRef<'cfg, 'ctx, Tag> {
    ptr: *mut sys::F641D,
    _tag: PhantomData<Tag>,
    _ctx: PhantomData<&'ctx Context<'cfg, Tag>>,
}
impl<Tag> Clone for F64Array1DRef<'_, '_, Tag> {
    fn clone(&self) -> Self {
        *self
    }
}
impl<Tag> Copy for F64Array1DRef<'_, '_, Tag> {}
impl<'cfg, 'ctx, Tag> F64Array1DRef<'cfg, 'ctx, Tag> {
    pub fn new(ctx: &Context<'cfg, Tag>, data: &mut [f64]) -> Self {
        use std::convert::TryInto;
        F64Array1DRef {
            ptr: unsafe { sys::futhark_new_f64_1d(
                ctx.ptr,
                data.as_ptr() as *mut _,
                data.len().try_into().unwrap(),
            ) },
            _tag: PhantomData,
            _ctx: PhantomData,
        }
    }
}


#[macro_export]
macro_rules! context {
    ($v:vis $name:ident) => {
        $v struct $name;
        impl $name {
            fn claim<'a>(cfg: &'a ContextConfig) -> Option<Context<'a, $name>> {
            use core::sync::atomic::{AtomicBool, Ordering};
                static CLAIMED: AtomicBool = AtomicBool::new(false);
                if CLAIMED.compare_exchange(false, true, Ordering::SeqCst, Ordering::SeqCst).is_ok() {
                    Some(unsafe { Context::<$name>::new_unchecked(&cfg) })
                } else {
                    None
                }
            }
        }
    }
}

#[macro_export]
macro_rules! local_context {
    ($name:ident : $cfg:expr) => {{
        struct $name;
        unsafe { Context::<$name>::new_unchecked($cfg) }
    }}
}
