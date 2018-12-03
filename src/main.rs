#![allow(unused_imports)]
#![feature(stmt_expr_attributes)]

#[macro_use]
extern crate lazy_static;
extern crate regex;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate bincode;
extern crate reqwest;
extern crate crypto;
extern crate smallvec;

mod error;
#[macro_use]
mod framework;

use std::env;
use reqwest::Client;
use crate::framework::Framework;

pub(crate) use crate::error::Error;
pub(crate) use crate::error::Result;

macro_rules! main {
    ($($days:ident),+$(,)*) => {
        $(
            mod $days;
        )+
        fn main() {
            let mut fw = Framework::new();

            $(
                {
                    crate::$days::register_day(&mut fw);
                }
            )+;

            let client = Client::new();

            let args: Vec<String> = env::args().collect();
            match args.len() {
                1 => {
                    // execute all
                    $(
                        {
                            if let Err(e) = fw.execute(&client, stringify!($days)) {
                                eprintln!("{}", e);
                                std::process::exit(-2);
                            }
                        }
                    )+;
                },
                2 => {
                    // execute specific day
                    if let Err(e) = fw.execute(&client, args[1].as_str()) {
                        eprintln!("{}", e);
                        std::process::exit(-2);
                    }
                },
                _ => {
                    eprintln!("too many arguments");
                    std::process::exit(-1);
                }
            }
        }
    };
}

main!(
    day01,
    day02,
    day03,
);
