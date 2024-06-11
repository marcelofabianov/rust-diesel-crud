mod db;
mod models;
mod repo;
mod schema;

fn main() {
    let mut connection = db::establish_connection();

    repo::create_new_user(&mut connection);

    let new_user = repo::create_new_user(&mut connection);
    println!("User created: {:?}", new_user);

    let all_users = repo::get_all_users(&connection);
    println!("Users:");

    for user in all_users {
        println!("{:?}", user);
    }
}
