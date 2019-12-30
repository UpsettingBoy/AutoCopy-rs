/*
    Copyright 2019 Jerónimo Sánchez <jeronimosg@hotmail.es>

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

       http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.

    SPDX-License-Identifier: Apache-2.0
*/

use diesel::Queryable;
use diesel::Insertable;

use std::path::Path;
use std::error::Error;
use std::convert::Into;

use super::schema::folders;

#[derive(Queryable)]
pub struct Folder {
    pub id: i32,
    pub name: String,
    pub location: String,
    pub destiny: String,
    pub interval: i32
}

impl Folder {
    ///Realizes a recursive copy of [location] into [destiny] folder.
    pub fn do_copy(&self) -> Result<(), String> {
        let loc = Path::new(self.location.as_str());
        let des = Path::new(self.destiny.as_str());

        if !loc.exists() {
            return Err(format!("Original path {} do not exists!", loc.display()));
        }

        if !des.exists() {
            return Err(format!("Destiny path {} do not exists!", des.display()));
        }

        let mut opts = fs_extra::dir::CopyOptions::new();
        opts.overwrite = true;
        opts.copy_inside = true;

        match fs_extra::copy_items(&vec![loc.to_str().unwrap()], des.to_str().unwrap(), &opts) {
            Ok(_) => Ok(()),
            Err(err) => Err(err.description().to_string())
        }
    }
}

#[derive(Insertable)]
#[table_name = "folders"]
pub struct NewFolder {
    pub name: String,
    pub location: String,
    pub destiny: String,
    pub interval: i32
}