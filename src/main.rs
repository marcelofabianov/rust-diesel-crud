mod db;
mod models;
mod repo;
mod schema; // Importe o módulo repo.rs

fn main() {
    let connection = db::establish_connection();

    repo::create_new_user(&connection);

    let new_user = repo::create_new_user(&connection);
    println!("User created: {:?}", new_user);

    let all_users = repo::get_all_users(&connection);
    println!("Users:");

    for user in all_users {
        println!("{:?}", user);
    }
}
