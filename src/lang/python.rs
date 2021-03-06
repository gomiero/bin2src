// bin2src - convert a binary file to source code in various languages
//
//  Copyright (C) 2020  Alexandre Gomiero de Oliveira
//
//  This program is free software: you can redistribute it and/or modify
//  it under the terms of the GNU General Public License as published by
//  the Free Software Foundation, either version 3 of the License, or
//  (at your option) any later version.
//
//  This program is distributed in the hope that it will be useful,
//  but WITHOUT ANY WARRANTY; without even the implied warranty of
//  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
//  GNU General Public License for more details.
//
//  You should have received a copy of the GNU General Public License
//  along with this program.  If not, see <https://www.gnu.org/licenses/>.

use super::super::generator::GeneratorOutput;
use std::error::Error;
use std::fs;
use std::io::{BufWriter, Write};

pub struct Python {
    go: GeneratorOutput,
}

impl Python {
    pub fn new(g: GeneratorOutput) -> Self {
        Python { go: g }
    }

    fn out_header(&mut self, f: &mut BufWriter<fs::File>) -> Result<(), &'static str> {
        let mut doblock = move || -> Result<(), Box<dyn Error>> {
            writeln!(
                f,
                "#*******************************************************************************"
            )?;
            writeln!(f, "#")?;
            writeln!(f, "#      bin2src - https://github.com/gomiero/bin2src")?;
            writeln!(f, "#")?;
            writeln!(f, "#      File: {:63}", self.go.ifile_name)?;
            writeln!(f, "#")?;
            writeln!(f, "#  MIT License")?;
            writeln!(f, "# ")?;
            writeln!(
                f,
                "#  Copyright (c) 2020-2020 Alexandre Gomiero de Oliveira"
            )?;
            writeln!(f, "# ")?;
            writeln!(
                f,
                "#  Permission is hereby granted, free of charge, to any person obtaining a copy"
            )?;
            writeln!(f, "#  of this software and associated documentation files (the \"Software\"), to deal")?;
            writeln!(
                f,
                "#  in the Software without restriction, including without limitation the rights"
            )?;
            writeln!(
                f,
                "#  to use, copy, modify, merge, publish, distribute, sublicense, and/or sell"
            )?;
            writeln!(
                f,
                "#  copies of the Software, and to permit persons to whom the Software is"
            )?;
            writeln!(
                f,
                "#  furnished to do so, subject to the following conditions:"
            )?;
            writeln!(f, "# ")?;
            writeln!(
                f,
                "#  The above copyright notice and this permission notice shall be included in "
            )?;
            writeln!(f, "#   all copies or substantial portions of the Software.")?;
            writeln!(f, "# ")?;
            writeln!(
                f,
                "#  THE SOFTWARE IS PROVIDED \"AS IS\", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR"
            )?;
            writeln!(
                f,
                "#  IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,"
            )?;
            writeln!(
                f,
                "#  FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE"
            )?;
            writeln!(
                f,
                "#  AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER"
            )?;
            writeln!(
                f,
                "#  LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,"
            )?;
            writeln!(
                f,
                "#  OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE"
            )?;
            writeln!(f, "#  SOFTWARE.")?;
            writeln!(f, "# ")?;
            writeln!(
                f,
                "#*******************************************************************************"
            )?;
            writeln!(f)?;
            if self.go.hex {
                write!(f, "{}_DATA  = b\"", self.go.ofile_name.to_uppercase())?;
            } else {
                writeln!(f, "{}_DATA = bytes([", self.go.ofile_name.to_uppercase())?;
            };
            Ok(())
        };

        if let Err(_err) = doblock() {
            Err("Error when writing header")
        } else {
            Ok(())
        }
    }

    fn out_footer(&mut self, f: &mut BufWriter<fs::File>) -> Result<(), &'static str> {
        let mut doblock = move || -> Result<(), Box<dyn Error>> {
            if self.go.hex {
                write!(f, "\"")?;
            } else {
                writeln!(f, "])")?;
            };
            writeln!(f)?;
            Ok(())
        };
        if let Err(_err) = doblock() {
            Err("Error when writing data block")
        } else {
            Ok(())
        }
    }

    pub fn generate_files(&mut self) -> Result<(), &'static str> {
        let writeif = |hex: bool,
                       comma: bool,
                       f: &mut BufWriter<fs::File>,
                       expr: u8|
         -> Result<(), Box<dyn Error>> {
            if hex {
                write!(f, "\\x{:>02x}", expr)?;
            } else {
                if comma {
                    write!(f, "{:3}, ", expr)?;
                } else {
                    write!(f, "{:3}", expr)?;
                };
            }
            Ok(())
        };
        let sep = if self.go.hex {
            format!("\"\n{}_DATA += b\"", self.go.ofile_name.to_uppercase())
        } else {
            String::from("\n")
        };
        self.go.set_output_fname();
        self.go.odir_path.push(&self.go.ofile_name);
        self.go.odir_path.set_extension("py");

        let mut ofile: BufWriter<fs::File> = match fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.go.odir_path)
        {
            Ok(f) => BufWriter::with_capacity(32768, f),
            _ => return Err("Can't create output file"),
        };
        self.out_header(&mut ofile)?;
        self.go.write_data(&mut ofile, 16, writeif, sep)?;
        self.out_footer(&mut ofile)?;
        Ok(())
    }
}
