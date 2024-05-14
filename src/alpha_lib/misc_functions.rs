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


use std::io::{self, Write, BufWriter};


pub fn log_missed_symbol(buf_writer: &mut BufWriter<impl Write>, data: &str) -> io::Result<()> {
    let newln = format!("{}\n", data);
    buf_writer.write_all(newln.as_bytes()) // Convert string to bytes and write
}

//SHOP
// GLBE
// RBLX
// RXRX
// VERV
// SQ
// CRYPTO:BTC
// RDDT
// WBD
// MRX
// SHAK
// AWON
// CAVA
// XPEV
// LI
// SQNXF
// NTDOF
// CEJOF
// FOREX:JPY
// FOREX:USD
// MJNA
// SZKMF
// FOREX:JPY
// CRYPTO:BTC
// CRYPTO:DOGE
// CRYPTO:BTC
// MDNDF
// LKNCY
// DNZOF
// SAEOF
// VLEEF
