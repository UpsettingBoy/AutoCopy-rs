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

#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate diesel;

use clap::{clap_app, crate_version, crate_authors, ArgMatches};
use prettytable::row;
use prettytable::cell;
use diesel::{SqliteConnection, insert_into, RunQueryDsl, ExpressionMethods, QueryDsl};
use time::Time;

use std::thread::sleep;
use std::time::{Duration, SystemTime};
use std::thread;
use std::collections::HashMap;

use crate::utils::io::{initialize_folder, lock_execution};
use crate::utils::db::create_connection;
use crate::models::{Folder, NewFolder};


mod utils;
pub mod schema;
pub mod models;

fn main() {
    //Console help & commands info
    let matches = clap_app!(AutoCopy =>
        (version: crate_version!())
        (author: crate_authors!())
        (@subcommand start =>
            (about: "Starts the program")
            (@arg verbose: -v --verbose "Get detailed output")
        )
        (@subcommand show =>
            (about: "Shows all added profiles")
        )
        (@subcommand remove =>
            (about: "Removes the selected profile")
            (@arg PROFILE_ID: +required "ID of the profile")
        )
        (@subcommand add =>
            (about: "Adds a folder to the copy registry (creates a profile)")
            (@arg NAME: +required "Name of the profile")
            (@arg LOCATION: +required "Absolute path of the folder to copy")
            (@arg DESTINY: +required "Absolute path where store the copied folder")
            (@arg INTERVAL: +required "Interval between copies (in seconds)")
        )
    ).get_matches();

    initialize_folder();

    let conn = create_connection();

    //Subcommand matching
    match matches.subcommand() {
        ("add", args) => add_entry(&conn, args.expect("No args found!")),
        ("show", _) => display_table(&conn),
        ("remove", args) => remove_profile(&conn, args.expect("No args found!")),
        ("start", args)  => start_command(&conn, args.unwrap().is_present("verbose")),
        _ => start_command(&conn, false)
    }
}

///Copies folders in determined time intervals.
fn start_command(conn: &SqliteConnection, verbose: bool) {
    use schema::folders::dsl::*;

    let query = folders
        .load::<Folder>(conn)
        .expect("Could not load database content!");

    //Grouping profiles by time interval
    let mut map: HashMap<i32, Vec<Folder>> = HashMap::new();
    for row in query {
        let value = map.get_mut(&row.interval);
        
        if let Some(value_un) = value {
            value_un.push(row);
        }
        else {
            map.insert(row.interval, vec![row]);
        }
    }

    //Threads handles
    let mut handles = Vec::with_capacity(map.len());

    for (key, value) in map {
        handles.push(thread::spawn(move || { //<- Thread initialization
            loop {
                let start = SystemTime::now();
                let format_time = Time::now();

                for profile in &value {
                    if verbose {
                        println!("[{}] Copying profile {}", format_time.format("%T"), profile.name);
                    }

                    match profile.do_copy() {
                        Ok(_) => (),
                        Err(err) => {
                            println!("Profile {} error: {}",profile.name , err);
                            break;
                        }
                    }

                    if verbose {
                        println!("Profile {} took {:#?}", profile.name, start.elapsed().unwrap());
                    }
                }
                
                sleep(Duration::from_secs(key as u64));
            }
        }));
    }

    //Locking main thread from exiting
    lock_execution(|| {
        loop {
            sleep(Duration::from_secs(5));
        }
    })
}

///Removes a entry of the database.
fn remove_profile(conn: &SqliteConnection, mat: &ArgMatches) {
    use schema::folders::dsl::*;

    let search_id = mat.value_of("PROFILE_ID").unwrap().parse::<i32>().expect("PROFILE_ID must be a valid number!");

    let delete = diesel::delete(folders.filter(id.eq(search_id)))
        .execute(conn)
        .unwrap_or_else(|_err| panic!("Error deleting profile with ID = {}", search_id));

    if delete == 0 {
        println!("There is no profile with ID = {}", search_id);
    }
}

///Displays the database content. If is empty, shows nothing.
fn display_table(conn: &SqliteConnection) {
    use schema::folders::dsl::*;

    let query = folders
        .load::<Folder>(conn)
        .expect("Could not load database content!");

    let mut table = prettytable::Table::new();
    table.add_row(row!["ID", "Profile", "Location", "Destiny", "Interval"]);

    for row in &query {
        table.add_row(row![row.id, row.name, row.location, row.destiny, format!("{} s", row.interval)]);
    }

    if query.is_empty() {
        table.printstd();
    }
}

///Adds a new entry into the database.
fn add_entry(conn: &SqliteConnection, mat: &ArgMatches) {
    use schema::folders;

    let name = mat.value_of("NAME").unwrap();
    let location = mat.value_of("LOCATION").unwrap();
    let destiny = mat.value_of("DESTINY").unwrap();
    let interval = mat.value_of("INTERVAL").unwrap().parse::<i32>().expect("INTERVAL must be a valid number!");

    let entry = NewFolder {
        name: name.to_string(),
        location: location.to_string(),
        destiny: destiny.to_string(),
        interval
    };

    let exec = insert_into(folders::table)
        .values(entry)
        .execute(conn);

    match exec {
        Ok(_tam) => (),
        Err(_err) => println!("Error adding items to database!")
    }
}


