use std::fmt;
use std::sync::{Arc, Condvar, Mutex};
use std::time::Duration;

pub struct Latch {
  open: Mutex<bool>,
  condvar: Condvar,
}

impl Latch {
  pub fn new() -> Arc<Latch> {
    Arc::new(Latch {
      open: Mutex::new(false),
      condvar: Condvar::new(),
    })
  }

  // Open the latch: unblock the waits
  pub fn open(&self) {
    // Mutex.lock(): acquires a mutex, blocking the current thread until success
    // A guard is return from Mutex.lock(). When the guard goes out of scope, the mutex will be unlocked.
    let mut open = self.open.lock().unwrap();
    *open = true;
    // Wakes up one blocked thread on this condvar
    self.condvar.notify_one();
  }

  pub fn wait(&self) {
    let mut open = self.open.lock().unwrap();
    while !*open {
      open = self.condvar.wait(open).unwrap();
    }
  }

  pub fn opened(&self) -> bool {
    *self.open.lock().unwrap()
  }
}