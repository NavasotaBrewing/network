#![allow(dead_code)]
#![feature(proc_macro_hygiene, decl_macro, drain_filter)]
use std::env;
use std::process::exit;


mod api;
mod model;

fn help() {
    println!("Enter one of the following: ");
    println!("master:  Starts the master API");
    println!("rtu:     Starts the RTU API");
}



#[tokio::main]
async fn main() {
    if cfg!(features = "rtu") {
        println!("RTU Mode Enabled");
    } else {
        println!("RTU Mode Disabled. No hardware interaction will be made");
    }

    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        help();
        exit(0);
    }

    

    match args[1].as_str() {
        // "master" => crate::api::master::run(),
        "rtu" => crate::api::RTU::run().await,
        _ => {
            eprintln!("Arg '{}' not known.", args[1].as_str());
            help();
            exit(1);
        }
    };
}
