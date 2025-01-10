/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use std::io::{BufRead, Read, Seek, Write};

pub fn unzip(zip_reader: impl Read) -> Result<Box<dyn BufRead>, ZipError> {
    todo!()
}

fn zip(csv_reader: impl Read, zip_writer: impl Write + Seek) {
    todo!()
}

pub struct ZipError;
