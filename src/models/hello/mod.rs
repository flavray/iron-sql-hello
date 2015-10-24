use time;

use models::SqlPooledConnection;

#[derive(Debug, RustcDecodable, RustcEncodable)]
pub struct Hello {
    pub id: i32,
    pub content: String,
    pub created_at: time::Timespec
}

impl Hello {
    pub fn new(content: String) -> Hello {
        Hello {
            id: 0,
            content: content,
            created_at: time::get_time()
        }
    }

    pub fn all(connection: SqlPooledConnection) -> Vec<Hello> {
        let mut statement = connection.prepare("SELECT * FROM hellos").unwrap();
        let mut result = Vec::new();

        match statement.query(&[]) {
            Ok(query) => {
                for hello_row in query {
                    if hello_row.is_err() { continue; }
                    let row = &hello_row.unwrap();

                    result.push(Hello {
                        id: row.get(0),
                        content: row.get(1),
                        created_at: row.get(2)
                    });
                }
            }
            Err(_) => ()  // result will be empty
        }

        result
    }

    pub fn get(id: i32, connection: SqlPooledConnection) -> Option<Hello> {
        let mut statement = connection.prepare("SELECT * FROM hellos WHERE id = $1 LIMIT 1").unwrap();

        match statement.query(&[&id]) {
            Ok(query) => {
                for hello_row in query {
                    if hello_row.is_err() { continue; }
                    let row = &hello_row.unwrap();

                    return Some(Hello {
                        id: row.get(0),
                        content: row.get(1),
                        created_at: row.get(2)
                    });
                }
            }
            Err(_) => ()  // flow will continue and None will be returned
        }

        None
    }

    pub fn create(&mut self, connection: SqlPooledConnection) -> Option<&Hello> {
        let result = connection.execute("INSERT INTO hellos (content, created_at) VALUES ($1, $2)", &[
            &self.content,
            &self.created_at
        ]);

        match result {
            Ok(_) => Some(self),
            Err(_) => None
        }
    }

    pub fn delete(id: i32, connection: SqlPooledConnection) -> bool {
        match connection.execute("DELETE FROM hellos WHERE id = $1", &[&id]) {
            Ok(updated) => updated > 0,
            Err(_) => false
        }
    }
}
