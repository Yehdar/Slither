extern crate html5ever;
extern crate url;

use std::env;

mod parse;
mod fetch;
mod crawler;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        let start_url_string = &args[1];
    }
}
