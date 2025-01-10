/*
    Created by Zoltan Kovari, 2025.

    Licensed under the Apache License, Version 2.0
    http://www.apache.org/licenses/LICENSE-2.0
    (see LICENSE.txt)
*/

use std::io::Read;

fn download_latest() {
    todo!();
}

fn download_with_path<T>(path: &str) -> Result<Box<T>, DownloadError>
where T: Read
{
    todo!()
}

struct DownloadError;
