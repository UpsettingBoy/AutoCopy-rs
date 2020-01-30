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

use app_dirs2::AppInfo;

const DATABASE: &str = "data.sqlite";
const LOCK_FILE: &str = "lock.data";
const APP_DATA: AppInfo = AppInfo {
    name: "AutoCopy",
    author: "UpsettingBoy",
};

///! Module with database operations
pub mod db {

    use diesel::{Connection, SqliteConnection};

    use crate::error::AutoCopyError;
    use crate::utils::io::get_database_path;

    embed_migrations!();

    ///Returns a new connection to the database.
    /// This also runs all pending migrations.
    ///
    /// # Example
    /// ```
    ///let conn = create_connection();
    /// ```
    pub fn create_connection() -> Result<SqliteConnection, AutoCopyError> {
        let conn = SqliteConnection::establish(get_database_path()?.to_str().unwrap())
            .expect("No connection to db");

        embedded_migrations::run(&conn).expect("Could not create database schema!");

        Ok(conn)
    }
}

///! Contains various operations over files
pub mod io {
    use app_dirs2::{get_app_root, AppDataType};
    use fs2::FileExt;

    use std::error::Error;
    use std::fs;
    use std::fs::File;
    use std::path::PathBuf;

    use crate::error::AutoCopyError;
    use crate::utils::{APP_DATA, DATABASE, LOCK_FILE};

    ///Initializes folders and files needed for this app to work.
    pub fn initialize_folder() -> Result<(), AutoCopyError> {
        let path = get_dir()?;
        let db_path = get_database_path()?;
        let lock_path = get_lockfile_path()?;

        fs::create_dir_all(&path)?;

        if !db_path.exists() {
            File::create(db_path)?;
        }

        if !lock_path.exists() {
            File::create(lock_path)?;
        }

        Ok(())
    }

    ///Returns the path to app config/data folder.
    pub fn get_dir() -> Result<PathBuf, AutoCopyError> {
        match get_app_root(AppDataType::UserConfig, &APP_DATA) {
            Ok(path) => Ok(path),
            Err(error) => Err(AutoCopyError::ConfigFolderError(
                error.description().to_string(),
            )),
        }
    }

    ///Returns the path to the database file.
    pub fn get_database_path() -> Result<PathBuf, AutoCopyError> {
        Ok(get_dir()?.join(DATABASE))
    }

    ///Returns the path to the program *lock* file.
    pub fn get_lockfile_path() -> Result<PathBuf, AutoCopyError> {
        Ok(get_dir()?.join(LOCK_FILE))
    }

    ///Locks the *lock* file and performs the given function.
    ///
    /// # Arguments
    /// * 'func' - A function 'Fn' to be executed during the lock.
    ///
    /// # Examples
    /// ```
    /// lock_execution(|| {
    //        //stuff to lock
    //    });
    /// ```
    pub fn lock_execution<F>(func: F)
    where
        F: Fn(),
    {
        let lock = File::open(get_lockfile_path().unwrap()).expect("Could not open lock file!");

        lock.try_lock_exclusive()
            .expect("Another instance of AutoCopy is running");
        func();
        lock.unlock().expect("Error occurred while unlocking file!");
    }
}
