use diesel::prelude::*;
use dotenvy::dotenv;
use schema::users::{email, username};
use std::env;
use self::models::{NewUser, User};

pub mod models;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}



pub fn create_user(
    conn: &mut PgConnection,
    first_name: &str,
    last_name: &str,
    username: &str,
    email: &str
) -> User {
    use crate::schema::users;

    let new_post = NewUser { first_name, last_name, username: &username, email };

    diesel::insert_into(users::table)
        .values(&new_post)
        .returning(User::as_returning())
        .get_result(conn)
        .expect("Error saving new post")
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
