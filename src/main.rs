#![feature(old_io,old_path,collections)]
#![allow(dead_code,unused_imports,deprecated)]
extern crate iron;
extern crate rusqlite;

use iron::prelude::*;
use iron::status;
// use core::fmt::Display;
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
    // fn hello_world(_: &mut Request) -> IronResult<Response> {
    //     Ok(Response::with((status::Ok, "Hello World!")))
    // }

    // println!("Serving on localhost:3000...");
    // Iron::new(hello_world).http("localhost:3000").unwrap();

    let mapper_path = Path::new("/Users/tristan/Box/Dev/Projects/wikicrush/data/xindex.db");
    // let bin_file = "/Users/tristan/Documents/WikiData/indexbi.bin";
    let bin_file = "/Users/tristan/Box/Dev/Projects/wikicrush/data/indexbi.bin";
    let mapper = mapping::Mapper::new(mapper_path);
    let mut graph = wiki::load_bin_graph(bin_file).ok().unwrap();

    let m_s_path = integ::find_path(&mut graph,&mapper,"alphabet","a");
    match m_s_path {
        Ok(path) => print_vec(&path[..]),
        Err(s) => println!("Error: {}",s),
    }
}
