use crate::models::*;
use diesel::prelude::*;

pub fn create_new_user(connection: &mut PgConnection) -> User {
    use crate::schema::users;

    let new_uuid = uuid::Uuid::new_v4();
    let new_user = NewUser::new(
        new_uuid,
        String::from("Marcelo Fabiano"),
        String::from("marcelofabianov@gmail.com"),
        String::from("password"),
    );

    diesel::insert_into(users::table)
        .values(&new_user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new user")
}

pub fn get_all_users(connection: &mut PgConnection) -> Vec<User> {
    use crate::schema::users::dsl::*;

    users.load::<User>(connection).expect("Error loading users")
}
