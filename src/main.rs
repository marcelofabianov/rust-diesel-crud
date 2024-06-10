use diesel::connection::SimpleConnection;

mod db;

fn main() {
    let mut conn = db::establish_connection();

    match conn.batch_execute("SELECT 1") {
        Ok(_) => println!("Ok"),
        Err(_) => eprintln!("Error connecting database"),
    }
}
