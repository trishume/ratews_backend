#![feature(old_io,old_path,collections)]
#![allow(dead_code,unused_imports,deprecated)]
extern crate iron;
extern crate rusqlite;
extern crate mount;

use iron::prelude::*;
use iron::status;
use mount::Mount;
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

fn api_find_scale(req: &mut Request) -> IronResult<Response> {
    Ok(Response::with((status::Ok, "Hello!")))
}

fn main() {
    let base_path = Path::new("/Users/tristan/Box/Dev/Projects/wikicrush/data/");
    let mapper_path = base_path.join("xindex.db");
    let bin_path = base_path.join("indexbi.bin");
    let bin_path_s = (*bin_path).to_str().unwrap();
    let mapper = mapping::Mapper::new(&*mapper_path);
    let mut graph = wiki::load_bin_graph(bin_path_s).ok().unwrap();

    let m_s_path = integ::find_path(&mut graph,&mapper,"alphabet","a");
    match m_s_path {
        Ok(path) => print_vec(&path[..]),
        Err(s) => println!("Error: {}",s),
    }

    let mut mount = Mount::new();
    // mount.mount("/", Static::new(Path::new("static/")));
    mount.mount("/api/findscale", api_find_scale);

    println!("Starting server on port 3000...");
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
