use clap::{App, Arg};

fn main(){
    let matches = App::new("echor")  // Constructor pattern?
        .version("0.1.0")
        .author("Stephen Akuffo from Ken YC")
        .about("Implements echo in rust")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();

    
        println!("{:#?}", matches);
}