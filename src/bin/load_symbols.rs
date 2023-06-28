use dotenvy::dotenv;
use std::{env, error::Error, process};

fn  main() {
    dotenv().ok();
    let file_list: Vec<(&str, &str)> = vec![("NYSE", "OTHER_LISTED"), ("NASDAQ", "NASDAQ_LISTED")];

    for file in file_list {
        println!("Loading {} symbols",file.0);
        let  filename:String = env::var(file.1).expect("No Data file found!");
    }


}