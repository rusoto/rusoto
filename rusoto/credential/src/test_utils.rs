#![cfg(test)]

use std::fmt::Debug;

pub const SECRET: &str = &"TtnuieannGt2rGuie2t8Tt7urarg5nauedRndrur";

pub fn is_secret_hidden_behind_asterisks<D>(obj: &D) -> bool
where
    D: Debug,
{
    let debug = format!("{:?}", obj);
    !debug.contains(SECRET) && debug.contains("**********")
}
