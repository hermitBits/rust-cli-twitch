pub mod db;

fn main() {
    let database: Vec<db::Channel> = db::load_database();
    dbg!(&database[0]);
}