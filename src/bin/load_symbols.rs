#[cfg(not(tarpaulin_include))]
use std::process;

use alpha_vantage_rust::{alpha_lib::alpha_io_funcs::process_symbols, file_processors::file_proc};
use dotenvy::dotenv;
///
/// This is the first program to be run in the alpha_vantage_rust project.
/// It populates the database with the symbols and names of the instruments
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

  let res = process_symbols(sec_vec, false);
  let _ = match res {
    Ok(_) => println!("Operation completed successfully."),
    Err(e) => println!("An error occurred: {}", e),
  };
}
