#![feature(old_io,old_path,collections)]
#![allow(dead_code,unused_imports,deprecated)]
extern crate iron;
extern crate rusqlite;
extern crate mount;
extern crate urlencoded;
extern crate serialize;
extern crate persistent;

use iron::prelude::*;
use iron::status;
use persistent::Write;
use iron::typemap::Key;
use mount::Mount;
// use core::fmt::Display;
use std::path::Path;
use urlencoded::UrlEncodedQuery;
use serialize::json;

mod wiki;
mod algos;
mod mapping;
mod integ;

#[derive(Encodable)]
struct ScaleResponse {
    status : &'static str,
    path: Vec<String>,
}

fn print_vec<T: std::fmt::Display>(v: &[T]) {
    for i in v.iter() {
        println!("{}", i)
    }
}

#[derive(Copy)]
pub struct GlobalGraph;
impl Key for GlobalGraph { type Value = wiki::Graph; }
#[derive(Copy)]
pub struct GlobalMapper;
impl Key for GlobalMapper { type Value = mapping::Mapper; }

fn api_find_scale(req: &mut Request) -> IronResult<Response> {
    let params = match req.get_ref::<UrlEncodedQuery>() {
        Ok(hashmap) => hashmap,
        Err(_) => return Ok(Response::with((status::BadRequest, "Need params."))),
    };
    if !params.contains_key("start") || !params.contains_key("end") {
        return Ok(Response::with((status::BadRequest, "Missing parameter.")))
    }
    let p1 = params.get("start").unwrap();
    let p2 = params.get("end").unwrap();
    // let m_s_path = integ::find_path(&mut graph,&mapper,p1,p2);
    // let path = match m_s_path {
    //     Some(params) => params,
    //     None => return Ok(Response::with((status::Ok, "{'status':'no path'}"))),
    // };
    // let resp = ScaleResponse {status: "ok", path: path};
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

    let mut api_chain = Chain::new(api_find_scale);
    api_chain.link(Write::<GlobalGraph>::both(graph));
    api_chain.link(Write::<GlobalMapper>::both(mapper));

    let mut mount = Mount::new();
    // mount.mount("/", Static::new(Path::new("static/")));
    mount.mount("/api/findscale", api_chain);

    println!("Starting server on port 3000...");
    Iron::new(mount).http("127.0.0.1:3000").unwrap();
}
