extern crate rusqlite;
use rusqlite::SqliteConnection;
use rusqlite::SqliteStatement;
use std::path::Path;

pub struct Mapper {
    conn : SqliteConnection,
    // name_to_id : SqliteStatement<'a>,
    // id_to_name : SqliteStatement<'a>,
}

impl Mapper {
    pub fn new(db_path : &Path) -> Mapper {
        let conn = SqliteConnection::open(db_path).unwrap();
        // let name_to_id = conn.prepare("SELECT offset FROM pages WHERE title = ? LIMIT 1").unwrap();
        // let id_to_name = conn.prepare("SELECT title FROM pages WHERE offset = ? LIMIT 1").unwrap();
        // Mapper {conn: conn, name_to_id: name_to_id, id_to_name: id_to_name}
        Mapper {conn: conn}
    }

    pub fn title_to_id(&self, title : &str) -> Option<usize> {
        let mut q = self.conn.prepare("SELECT offset FROM pages WHERE title = ? LIMIT 1").unwrap();
        let mut rows = q.query(&[&title]).unwrap();
        match rows.next() {
            Some(row) => {
                let int_offset : i64 = row.unwrap().get(0);
                Some(int_offset as usize)
            },
            None => None,
        }
    }

    pub fn id_to_title(&self, offset : usize) -> Option<String> {
        let mut q = self.conn.prepare("SELECT title FROM pages WHERE offset = ? LIMIT 1").unwrap();
        let mut rows = q.query(&[&(offset as i64)]).unwrap();
        rows.next().map(|row| {
            row.unwrap().get(0)
        })
    }
}
