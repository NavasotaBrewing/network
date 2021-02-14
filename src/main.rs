#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro, drain_filter)]
use std::env;
use std::process::exit;

extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
extern crate rocket_cors;
extern crate reqwest;

mod api;
mod configuration;
mod model;

fn help() {
    println!("Enter one of the following: ");
    println!("master:  Starts the master API");
    println!("rtu:     Starts the RTU API");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        help();
        exit(0);
    }

    match args[1].as_str() {
        "master" => crate::api::master::run(),
        "rtu" => crate::api::RTU::run(),
        _ => {
            eprintln!("Arg '{}' not known.", args[1].as_str());
            help();
            exit(1);
        }
    }
}
