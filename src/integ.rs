use wiki;
use algos;
use wiki::Graph;
use mapping::Mapper;

pub fn find_path(graph : &mut Graph, mapper : &Mapper,
                 start : &str, stop : &str) -> Result<Vec<String>,&'static str> {
    let p1 = match mapper.title_to_id(start) {
        Some(p1) => p1,
        None => return Err("Can't find first page"),
    };
    let p2 = match mapper.title_to_id(stop) {
        Some(p1) => p1,
        None => return Err("Can't find second page"),
    };
    // println!("Searching from {} to {}", p1,p2);

    let m_bid_path = algos::shortest_bid_path(graph,p1,p2);
    let m_path = match m_bid_path {
        Some(path) => Some(path),
        None => {
            algos::clear_marks(graph);
            algos::shortest_path(graph,p1,p2)
        }
    };
    algos::clear_marks(graph);
    let m_s_path : Option<Vec<String>> = m_path.map(|path| {
        path.iter().map(|id| {mapper.id_to_title(*id).unwrap()}).collect()
    });
    match m_s_path {
        Some(path) => Ok(path),
        None => Err("Can't find path"),
    }
}
