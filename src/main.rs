extern crate clap;

use clap::App;

fn main() {
    App::new("Habitica")
        .version("0.1")
        .about("Unnoficial Habitica CLI")
        .author("Dawson Israel")
        .get_matches();
}
