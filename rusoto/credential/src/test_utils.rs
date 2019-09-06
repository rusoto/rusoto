#![cfg(test)]

use lazy_static::lazy_static;
use std::fmt::Debug;
use std::sync::{Mutex, MutexGuard};

pub const SECRET: &str = &"TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur";

pub fn is_secret_hidden_behind_asterisks<D>(obj: &D) -> bool
where
    D: Debug,
{
    let debug = format!("{:?}", obj);
    !debug.contains(SECRET) && debug.contains("**********")
}

// cargo runs tests in parallel, which leads to race conditions when changing
// environment variables. Therefore we use a global mutex for all tests which
// rely on environment variables.
lazy_static! {
    pub static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
}

// As failed (panic) tests will poison the global mutex, we use a helper which
// recovers from poisoned mutex.
pub fn lock<'a, T>(mutex: &'a Mutex<T>) -> MutexGuard<'a, T> {
    match mutex.lock() {
        Ok(guard) => guard,
        Err(poisoned) => poisoned.into_inner(),
    }
}
