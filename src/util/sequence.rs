use std::sync::atomic::{AtomicU64, Ordering};

pub struct Sequence(AtomicU64);

impl Sequence {
    pub fn new() -> Sequence {
        Sequence(AtomicU64::new(0))
    }
    pub fn next(&self) -> u64 {
        self.0.fetch_add(1, Ordering::SeqCst)
    }
}

lazy_static! {
  pub static ref SEQUENCE: Sequence = Sequence::new();
}