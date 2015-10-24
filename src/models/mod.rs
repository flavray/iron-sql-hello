pub mod hello;

use iron::middleware::Chain;
use iron::typemap::Key;

use persistent::Read;

use r2d2::{Config, Pool, PooledConnection};
use r2d2_sqlite::SqliteConnectionManager;

pub type SqlitePool = Pool<SqliteConnectionManager>;
pub type SqlPooledConnection = PooledConnection<SqliteConnectionManager>;

pub struct AppDb;
impl Key for AppDb { type Value = SqlitePool; }

fn get_connection_pool() -> Pool<SqliteConnectionManager> {
    let manager = SqliteConnectionManager::new("db.sqlite3").unwrap();
    let config = Config::builder().pool_size(2).build();
    Pool::new(config, manager).unwrap()
}

pub fn database_middleware(mut middleware: Chain) -> Chain {
    let pool = get_connection_pool();
    middleware.link(Read::<AppDb>::both(pool));
    middleware
}
