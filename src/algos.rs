use wiki::Graph;
use std::collections::VecDeque;

fn flood_fill(graph : &mut Graph, start_page : usize, mark : u32) -> u32 {
    assert!(mark != 0);
    let mut stack = vec![start_page];
    let mut marked_count = 0;
    while !stack.is_empty() {
        let page = stack.pop().unwrap();

        if graph.user_data(page) != 0 {continue;}
        graph.set_user_data(page,mark); // mark visited
        // println!("Visiting {} with {} links",page,graph.bid_link_count(page));
        marked_count += 1;

        for linked in graph.bid_links(page) {
            // println!("Pushing link to {}", linked);
            stack.push(linked);
        }
    }
    marked_count
}

pub fn find_conn_components(graph : &mut Graph) {
    let mut start_page = graph.first_page();
    let mut comp_count = 0;
    loop {
        let count = flood_fill(graph, start_page,1);
        if count > 100 {
            println!("Found a connected component of {} nodes out of {} pages = {}.",
                     count,graph.page_count(),(count as f32 / graph.page_count() as f32));
        }
        comp_count += 1;

        let next_page = graph.find_next_unmarked(start_page);
        match next_page {
            Some(page) => start_page = page,
            None => break,
        }
    }
    println!("Found {} components.",comp_count);
}

pub fn clear_marks(graph : &mut Graph) {
    let mut start_page = graph.first_page();
    loop {
        graph.set_user_data(start_page,0);
        let next_page = graph.find_next(start_page);
        match next_page {
            Some(page) => start_page = page,
            None => break,
        }
    }
}

fn fill_incoming_links(graph : &mut Graph) {
    let mut page = graph.first_page();
    // Increment link count on all linked to pages, then move to next
    loop {
        for linked in graph.links(page) {
            let incd = graph.user_data(linked)+1;
            graph.set_user_data(linked, incd);
        }

        match graph.find_next(page) {
            None => break,
            Some(new_page) => page = new_page,
        }
    }
}

static DATA_HIST_MAX : usize = 50;
fn analyze_user_data(graph : &Graph) {
    let mut hist : Vec<u32> = vec![0; DATA_HIST_MAX];
    for page in graph.pages() {
        let count = graph.user_data(page);
        if (count as usize) < DATA_HIST_MAX {
            hist[count as usize] += 1;
        }
    }
    println!("Incoming links:");
    for c in 0..hist.len() {
        println!("{}: {}",c, hist[c]);
    }
}

pub fn shortest_path(graph : &mut Graph, start_page : usize, end_page : usize) -> Option<Vec<usize>> {
    let mut queue = VecDeque::new();
    queue.push_back(start_page);
    loop {
        if queue.is_empty() {
            return None;
        }
        let page = queue.pop_front().unwrap();
        if page == end_page {
            break;
        }

        for linked in graph.bid_links(page) {
            if graph.user_data(linked) == 0 {
                graph.set_user_data(linked, page as u32); // parent pointer
                queue.push_back(linked);
            }
        }
    }
    // kk we found the thing, reconstruct the path
    let mut cur_page : usize = end_page;
    let mut path = vec![end_page];
    while cur_page != start_page {
        cur_page = graph.user_data(cur_page) as usize; // work back
        path.insert(0,cur_page);
    }
    return Some(path);
}
