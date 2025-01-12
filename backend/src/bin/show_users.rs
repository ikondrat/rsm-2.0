use self::models::*;
use diesel::prelude::*;
use rsm_backend::*;

fn main() {
    use self::schema::users::dsl::*;

    let connection = &mut establish_connection();
    let results = users
        .limit(5)
        .select(User::as_select())
        .load(connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for user in results {
        println!("{}", user.id);
        println!("-----------\n");
        println!("{}", user.first_name);
    }
}