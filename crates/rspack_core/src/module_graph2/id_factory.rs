use std::sync::atomic::{AtomicUsize, Ordering};

pub struct IdFactory {
  next_id: AtomicUsize,
}

impl IdFactory {
  pub fn new() -> Self {
    Self {
      next_id: AtomicUsize::new(1),
    }
  }

  pub fn generate(&self) -> usize {
    self.next_id.fetch_add(1, Ordering::Relaxed)
  }
}
