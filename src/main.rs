#![feature(old_io,old_path,collections)]
#![allow(dead_code,unused_imports,deprecated)]
extern crate rusqlite;

use std::path::Path;

mod wiki;
mod algos;
mod mapping;
mod integ;

fn print_vec<T: std::fmt::Display>(v: &[T]) {
    for i in v.iter() {
        println!("{}", i)
    }
}

fn main() {
    let base_path = Path::new("/Users/tristan/Documents/WikiData/");
    // let base_path = Path::new("/Users/tristan/Box/Dev/Projects/wikicrush/data/");
    println!("log|Loading db");
    let mapper_path = base_path.join("xindex.db");
    let bin_path = base_path.join("indexbi.bin");
    let bin_path_s = (*bin_path).to_str().unwrap();
    let mapper = mapping::Mapper::new(&*mapper_path);
    println!("log|Loading graph");
    let mut graph = wiki::load_bin_graph(bin_path_s).ok().unwrap();
    println!("log|Done loading!");

    loop {
        let input = std::old_io::stdin().read_line().ok().expect("Failed to read line");
        let procd : Vec<&str> = input.trim().split_str("|").collect();
        if procd[0] == "godie" {
            break;
        }
        if procd.len() != 2 {
            continue;
        }

        let m_s_path = integ::find_path(&mut graph,&mapper,procd[0],procd[1]);
        match m_s_path {
            Ok(path) => println!("path|{}",path.connect("|")),
            Err(s) => println!("error|{}",s),
        }
    }
}
