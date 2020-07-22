#[repr(C)]
pub struct ContextConfig {
    private: [u8; 0],
}
#[repr(C)]
pub struct Context {
    private: [u8; 0],
}
#[repr(C)]
pub struct F641D {
    private: [u8; 0],
}
extern "C" {
    pub fn futhark_context_config_new() -> *mut ContextConfig;
    pub fn futhark_context_new(cfg: *mut ContextConfig) -> *mut Context;
    pub fn futhark_new_f64_1d(ctx: *mut Context, data: *mut f64, dim: i64) -> *mut F641D;
    pub fn futhark_free_f64_1d(ctx: *mut Context, arr: *mut F641D) -> i32;
    pub fn futhark_entry_avg(ctx: *mut Context, out: *mut f64, input: *const F641D) -> i32;
    pub fn futhark_context_sync(ctx: *mut Context) -> i32;
}
