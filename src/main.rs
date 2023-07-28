mod cmd;
mod declaration_parser;

use std::{
    fs::File,
    io::{BufRead, BufReader, BufWriter, Write},
};

use clap::Parser;
use cmd::Cli;
use declaration_parser::Declaration;

fn main() {
    let cli = Cli::parse();
    // println!("{:?}", cli);

    let file = File::open(&cli.input).unwrap();

    let reader = BufReader::new(file);

    let mut decls = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let decl = Declaration::from_str(&line);
        // println!("{:?}", decl);
        decls.push(decl);
    }
    decls.sort_by(|a, b| {
        if a.mod_size == b.mod_size {
            return a.weight.cmp(&b.weight);
        }
        b.mod_size.cmp(&a.mod_size)
    });
    if cli.output.is_some() {
        let file = File::create(cli.output.unwrap()).unwrap();
        let mut writer = BufWriter::new(file);
        for line in decls {
            writeln!(writer, "{}", line.line).ok();
        }
    } else {
        for line in decls {
            println!("{}", line.line);
        }
    }
}
