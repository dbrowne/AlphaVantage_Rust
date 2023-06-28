/*
 *
 *
 *
 *
 * MIT License
 * Copyright (c) 2023. Dwight J. Browne
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



use crate::datatypes::data_file_types::{NasdaqListed,NyseOtherSymbol};
use std::{error::Error};




fn csv_reader(files: Vec<(&str, String)>) -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let mut records: Vec<Vec<String>> = Vec::new();
    for (name, file_path) in files {
        let records_for_file = file_proc(&file_path, name)
            .map_err(|e| format!("Error processing file '{}': {}", name, e))?;
        records.push(records_for_file);
    }
    Ok(records)
}

fn file_proc(file_name: &str, exch_type: &str) -> Result<Vec<String>, Box<dyn Error>> {
    const NYSE: &str = "NYSE";
    const NASDAQ: &str = "NASDAQ";
    const MAX_RECORDS: usize = 15_000;

    let mut securities: Vec<String> = Vec::with_capacity(MAX_RECORDS);

    let mut rdr = csv::Reader::from_path(file_name)?;

    match exch_type {
        NYSE => {
            for result in rdr.deserialize() {
                let record: NyseOtherSymbol = result?;
                securities.push(record.actsymbol);
            }
        }
        NASDAQ => {
            for result in rdr.deserialize() {
                let record: NasdaqListed = result?;
                securities.push(record.symbol);
            }
        }
        _ => return Err(format!("Unknown exchange type: {}", exch_type).into()),
    }

    Ok(securities)
}


