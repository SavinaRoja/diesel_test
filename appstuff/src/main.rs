use diesel::prelude::*;
use dbstuff::models::post::Post;
use dbstuff::establish_connection;
use dbstuff::schema::post;

fn main() {
    let connection = establish_connection();

    let results = post::table
        .load::<Post>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} post(s)", results.len());
    for post in results {
        println!("{:?}", post);
    }
}
