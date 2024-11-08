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

use std::{
  env,
  io::{self, BufRead, BufWriter, Write},
};

pub fn log_missed_symbol(buf_writer: &mut BufWriter<impl Write>, data: &str) -> io::Result<()> {
  let newln = format!("{}\n", data);
  buf_writer.write_all(newln.as_bytes()) // Convert string to bytes and write
}

pub fn read_missed_symbols(file_name: String) -> io::Result<Vec<String>> {
  let mut missed_symbols = Vec::new();
  let file = std::fs::File::open(file_name)?;
  let reader = io::BufReader::new(file);
  for line in reader.lines() {
    missed_symbols.push(line?);
  }
  Ok(missed_symbols)
}

pub fn get_exe_name() -> String {
  let exe_name = env::current_exe().unwrap();
  exe_name.to_str().unwrap().to_string()
  // let exe_name = exe_name.file_name().unwrap();
  // let exe_name = exe_name.to_str().unwrap();
  // exe_name.to_string()
}
