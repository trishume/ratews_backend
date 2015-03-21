use std::env;
use std::mem;
use std::old_io::File;

static FILE_HEADER_SIZE : usize = 4*4;
static PAGE_HEADER_SIZE : usize = 3;

static PAGE_USER_DATA : usize = 0;
static PAGE_LINKS : usize = 1;
static PAGE_BID_LINKS : usize = 2;

pub struct Graph {
    data : Vec<u32>,
}

pub struct PageIter<'a> {
    g : &'a Graph,
    cur : usize,
}

impl<'a> Iterator for PageIter<'a> {
    type Item = usize;
    fn next(&mut self) -> Option<usize> {
        let next_page = self.cur + (PAGE_HEADER_SIZE+self.g.link_count(self.cur))*4;
        self.cur = next_page;
        if next_page >= self.g.data.len() * 4 { None } else { Some(next_page) }
    }
}

impl Graph {
    pub fn first_page(&self) -> usize {
        FILE_HEADER_SIZE
    }

    pub fn find_next(&self, page : usize) -> Option<usize> {
        let next_page = page + (PAGE_HEADER_SIZE+self.link_count(page))*4;
        if next_page >= self.data.len() * 4 { None } else { Some(next_page) }
    }

    pub fn find_next_unmarked(&self,start : usize) -> Option<usize> {
        let mut page = start;
        while self.user_data(page) != 0 {
            page = page + (PAGE_HEADER_SIZE+self.link_count(page))*4;
            if page >= self.data.len() * 4 { return None;}
        }
        Some(page)
    }

    pub fn pages(&self) -> PageIter {
        PageIter {g: self, cur: self.first_page()}
    }

    pub fn page_count(&self) -> u32 {
        self.data[1]
    }

    pub fn link_count(&self, page : usize) -> usize {
        self.data[page/4+PAGE_LINKS] as usize
    }

    pub fn bid_link_count(&self, page : usize) -> usize {
        self.data[page/4+PAGE_BID_LINKS] as usize
    }

    pub fn links(&self, page : usize) -> Vec<usize> {
        let start = page/4+PAGE_HEADER_SIZE;
        let end = start+self.link_count(page);
        let link_range = &self.data[start..end];
        link_range.iter().map(|x| *x as usize).collect::<Vec<usize>>()
    }

    pub fn bid_links(&self, page : usize) -> Vec<usize> {
        let start = page/4+PAGE_HEADER_SIZE;
        let end = start+self.bid_link_count(page);
        let link_range = &self.data[start..end];
        link_range.iter().map(|x| *x as usize).collect::<Vec<usize>>()
    }


    pub fn set_user_data(&mut self, page : usize, data : u32) {
        self.data[page/4+PAGE_USER_DATA] = data;
    }

    pub fn user_data(&self, page : usize) -> u32 {
        self.data[page/4+PAGE_USER_DATA]
    }
}


pub fn load_bin_graph(bin_path_s : &str) -> Result<Graph,&str> {
    // let args: Vec<String> = env::args().map(|x| x.to_string()).collect();

    // if args.len() != 2 {
    //     println!("Usage: ./strong_conn path/to/indexbi.bin");
    //     env::set_exit_status(1);
    //     return;
    // }

    let bin_path = Path::new(bin_path_s);
    println!("Analyzing {}...",bin_path.display());

    let mut file = File::open(&bin_path).ok().expect("Could not open graph file.");

    let mut graph_data : Vec<u32>;
    {
        let mut buf : Vec<u8> = file.read_to_end().ok().expect("Could not read file.");
        let len = buf.len();
        println!("Read {} bytes of file!", len);
        if len % 4 != 0 {
            return Err("Invalid file size!");
        }
        let data_ptr : *mut u32 = unsafe {mem::transmute(buf.as_mut_ptr())};
        graph_data = unsafe { Vec::from_raw_buf(data_ptr, len / 4)};
    }
    let graph = Graph { data: graph_data };
    println!("Read {} words of file!", graph.data.len());
    println!("Total pages: {}", graph.page_count());
    return Ok(graph);

    // find_conn_components(&mut graph);

    // println!("Finding incoming links...");
    // fill_incoming_links(&mut graph);
    // println!("Analyzing incoming links...");
    // analyze_user_data(&graph);
}
