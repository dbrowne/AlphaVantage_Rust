use dotenvy::dotenv;
use std::process;

use AlphaVantage_Rust::alpha_lib::alpha_io_funcs::process_symbols;
use AlphaVantage_Rust::file_processors::file_proc;

fn main() {
    dotenv().ok();
    let file_list: Vec<(&str, &str)> = vec![("NYSE", "OTHER_LISTED"), ("NASDAQ", "NASDAQ_LISTED")];

    let res = file_proc(file_list);

    let sec_vec = match res {
        Ok(vec) => vec,
        Err(e) => {
            println!("Error processing files: {}", e);
            process::exit(1);
        }
    };

    let res = process_symbols(sec_vec);
    let _ = match res {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("An error occurred: {}", e),
    };
}
