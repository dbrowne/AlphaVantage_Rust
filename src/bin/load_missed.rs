/*
 *
 *
 *
 *
 * MIT License
 * Copyright (c) 2024. Dwight J. Browne
 * dwight[-dot-]browne[-at-]dwightjbrowne[-dot-]com
 *
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the "Software"), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in all
 * copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
 * SOFTWARE.
 */
#[cfg(not(tarpaulin_include))]
use alpha_vantage_rust::alpha_lib::{
  alpha_io_funcs::process_symbols, misc_functions::read_missed_symbols,
};
use dotenvy::dotenv;

fn main() {
  dotenv().ok();
  if let Ok(secs) = read_missed_symbols("/tmp/symbol_log.txt".to_string()) {
    let mut symbs: Vec<Vec<String>> = Vec::new();
    symbs.push(secs);
    let res = process_symbols(symbs, true);
    let _ = match res {
      Ok(_) => println!("Operation completed successfully."),
      Err(e) => println!("An error occurred: {}", e),
    };
  } else {
    println!("Error reading missed symbols");
  }
}
