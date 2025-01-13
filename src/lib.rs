/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

pub mod error;
pub mod field;
pub mod format;
mod record;

mod csv;
mod download;
mod zip;

pub fn hello() {
    println!("Hello World!");
}
