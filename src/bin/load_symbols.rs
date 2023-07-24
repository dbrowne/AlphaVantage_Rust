use dotenvy::dotenv;
use std::{env, error::Error, fmt, process};
use AlphaVantage_Rust::datatypes::data_file_types::{NasdaqListed, NyseOtherSymbol};
use std::io::BufReader;
use std::fs::File;
use AlphaVantage_Rust::alpha_lib::alpha_io_funcs::process_symbols;

const MAX_SYMBOLS: usize = 10000;
const NASDAQ: &str = "NASDAQ";
const NYSE: &str = "NYSE";

fn main() {
    dotenv().ok();
    let file_list: Vec<(&str, &str)> = vec![("NYSE", "OTHER_LISTED"), ("NASDAQ", "NASDAQ_LISTED")];

    let  res = file_proc(file_list);

    let  sec_vec = match res {
        Ok(vec) => vec,
        Err(e) => {
            println!("Error processing files: {}", e);
            process::exit(1);
        }
    };

    let  res =process_symbols(sec_vec);
    let  _ = match res {
        Ok(_) => println!("Operation completed successfully."),
        Err(e) => println!("An error occurred: {}", e),
    };

}




#[derive(Debug, Clone)]
struct UnknownExchangeError(String);

impl fmt::Display for UnknownExchangeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Unknown exchange type: {}", self.0)
    }
}

impl Error for UnknownExchangeError {}


/// Processes the root exchange data files and returns the list of securities.
///
/// The function takes a vector of tuples, where each tuple contains a category and an environment variable identifier.
/// The environment variable identifier is used to retrieve the filename of a CSV file to be processed.
/// The category is passed to the file processing function `file_proc` along with the filename.
///
/// The file processing function `file_proc` is assumed to return a vector of strings.
/// These vectors are then collected into a larger vector, which is returned by the function.
///
/// # Arguments
///
/// * `file_arr` - Contains the NYSE and NASDAQ Seed files.
///
/// # Returns
///
/// * `Ok(Vec<Vec<String>>)` - A nested vector containing the security names from each file.
/// * `Err(Box<dyn Error>)` - An error if any occurred during the processing. This could be due to failure to read the environment variable, or failure in file processing.
///
/// # Panics
///
/// This function might panic if the `file_proc` function panics.
///
/// # Errors
///
/// This function will return an error if the environment variable cannot be read or if the `file_proc` function returns an error.
fn file_proc(file_arr: Vec<(&str, &str)>) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut security_vector: Vec<Vec<String>> = Vec::new();
    for items in file_arr {
        let file_name = match env::var(items.1) {
            Ok(filename) => filename,
            Err(e) => {
                println!("Couldn't read {}: {}", items.1, e);
                return Err(e.into());
            }
        };

        let file_data = csv_proc(file_name, items.0)?;
        security_vector.push(file_data);
    }

    return Ok(security_vector);
}

/// Opens and reads an exchange CSV file, deserializing each record into either a `NasdaqListed` or `NyseOtherSymbol` struct
/// based on the `exchange` parameter. Extracts the relevant symbol from each record and stores them in a vector.
///
/// # Arguments
/// * `file_name` - A `String` that holds the name of the CSV file to be processed.
/// * `exchange` - A string slice that denotes the type of exchange (NASDAQ or NYSE). This determines the type of struct that the CSV records are deserialized into.
///
/// # Returns
/// * `Ok(Vec<String>)` - A vector of `String` objects, each representing the symbol from a record in the CSV file. The vector has an initial capacity of `MAX_SYMBOLS`.
/// * `Err(Box<dyn Error>)` - A boxed dynamic Error object that might be returned in case of failure to open the file, failure to deserialize the records, or if an unknown exchange type is provided.
///
/// # Errors
/// This function will return an `Err` variant if the file cannot be opened, if there is an error during deserialization of the CSV records, or if an unknown exchange type is passed.
///
/// It can also return a `UnknownExchangeError` if the `exchange` parameter does not match with either NASDAQ or NYSE.
fn csv_proc(file_name: String, exchange: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let file = File::open(file_name)?;
    let mut file_reader = csv::Reader::from_reader(BufReader::new(file));
    let mut symbols: Vec<String> = Vec::with_capacity(MAX_SYMBOLS);

    match exchange {
        NASDAQ => {
            for result in file_reader.deserialize() {
                let record: NasdaqListed = result?;
                symbols.push(record.symbol);
            }
        }
        NYSE => {
            for result in file_reader.deserialize() {
                let record: NyseOtherSymbol = result?;
                symbols.push(record.actsymbol);
            }
        }
        _ => {
            println!("csv_proc: unknown exchange type: {}", exchange);
            return Err(Box::new(UnknownExchangeError(exchange.to_string())));
        }
    }
    Ok(symbols)
}