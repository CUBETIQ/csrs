use std::io::ErrorKind;
use std::path::Path;
use std::process::Command;

use clap::{App, Arg};

fn main() {
    let matches = App::new("CUBETIQ Syncer")
        .version("0.1.0")
        .author("Sambo Chea <sombochea@cubetiqs.com>")
        .about("For CUBETIQ software tools")
        .arg(Arg::with_name("name")
            .short("n")
            .long("name")
            .takes_value(true)
            .help("Enter your name to register the service."))
        .arg(Arg::with_name("app_id")
            .short("id")
            .long("app_id")
            .takes_value(true)
            .help("Enter your app id that register with your service."))
        .get_matches();

    let name = matches.value_of("name");
    let app_id = matches.value_of("app_id");

    match name {
        None => println!("User is not available for use this service!"),
        Some(s) => println!("Your name is: {}", s)
    }

    match app_id {
        None => println!("App Id is required to process the service!"),
        Some(s) => {
            println!("Your app id is: {}", s);
        }
    }

    let _b = check_mysqldump();
    println!("MySQL Dump is {}", _b);
    let _b1 = check_pg_dump();
    println!("Postgres Dump is {}", _b1);
}

fn check_is_windows() -> bool {
    return cfg!(windows)
}

fn check_mysqldump() -> bool {
    let mut mysqldump_file = "mysqldump".to_owned();
    if check_is_windows() {
        mysqldump_file.push_str(".exe");
    }
    return match Command::new(mysqldump_file).spawn() {
        Ok(_) => true,
        Err(e) => {
            if let _not_found = e.kind() {
                println!("`mysqldump` not found to execute, please add it into PATH!");
            }
            false
        },
    }
}

fn check_pg_dump() -> bool {
    let mut pg_dump_filename = "pg_dump".to_owned();
    if check_is_windows() {
        pg_dump_filename.push_str(".exe");
    }
    return match Command::new(pg_dump_filename).spawn() {
        Ok(_) => true,
        Err(e) => {
            if let _not_found = e.kind() {
                println!("`pg_dump` not found to execute, please add it into PATH!");
            }
            false
        },
    }
}