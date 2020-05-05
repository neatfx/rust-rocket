use std::sync::atomic::AtomicUsize;

pub struct HitCount {
    pub count: AtomicUsize,
}

impl HitCount {
    pub fn new() -> HitCount {
        HitCount {
            count: AtomicUsize::new(0),
        }
    }
}
