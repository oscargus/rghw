use ghw_sys::ghw_sig;

pub struct GHWSignal {
    pub handle: *mut ghw_sig,
}

impl GHWSignal {
    pub fn get_type(&self) {}
}
