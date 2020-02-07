#![cfg(test)]

use lazy_static::lazy_static;
use std::collections::HashMap;
use std::ffi::OsString;
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

// cargo runs tests in parallel, which leads to race conditions when changing environment
// variables. Therefore we use a global mutex for all tests which rely on environment variables.
//
// As failed (panic) tests will poison the global mutex, we use a helper which recovers from
// poisoned mutex.
//
// The first time the helper is called it stores the original environment. If the lock is poisoned,
// the environment is reset to the original state.
pub fn lock_env() -> MutexGuard<'static, ()> {
    lazy_static! {
        static ref ENV_MUTEX: Mutex<()> = Mutex::new(());
        static ref ORIGINAL_ENVIRONMENT: HashMap<OsString, OsString> =
            std::env::vars_os().collect();
    }

    let guard = ENV_MUTEX.lock();
    lazy_static::initialize(&ORIGINAL_ENVIRONMENT);
    match guard {
        Ok(guard) => guard,
        Err(poisoned) => {
            for (name, _) in std::env::vars_os() {
                if !ORIGINAL_ENVIRONMENT.contains_key(&name) {
                    std::env::remove_var(name);
                }
            }
            for (name, value) in ORIGINAL_ENVIRONMENT.iter() {
                std::env::set_var(name, value);
            }
            poisoned.into_inner()
        }
    }
}
