use clap::{Arg, App};

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
}
