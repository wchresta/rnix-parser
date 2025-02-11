use rnix::types::*;
use std::{io, io::Read};

fn main() {
    let mut content = String::new();
    io::stdin().read_to_string(&mut content).expect("could not read nix from stdin");
    let ast = rnix::parse(&content);

    for error in ast.errors() {
        println!("error: {}", error);
    }

    println!("{}", ast.root().dump());
}
