use std::env;

use dotenv::dotenv;

use diesel::prelude::*;
use diesel::SqliteConnection;

use super::schema::posts::dsl::{posts, published as posts_published, title as posts_title};
use super::models::{Post, NewPost};
use std::error::Error;
use diesel::connection::SimpleConnection;

embed_migrations!("./migrations");

pub fn establish_connection(database_url: String) -> SqliteConnection {
    dotenv().ok();

    /*
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    */
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}


pub fn do_some_db_op(database_url: String) {
    android_log::init("app_rust_sql").unwrap();
    log_panics::init();

    let connection = &establish_connection(database_url);

    db_migrations(connection);

    let postId = create_post(connection, "title001", "body001");

    publish_post(connection, postId as i32);

    show_posts(connection)
}

pub fn db_migrations(conn: &SqliteConnection) {
    //conn.batch_execute()

    // This will run the necessary migrations.
    embedded_migrations::run(conn);

    // By default the output is thrown out. If you want to redirect it to stdout, you
    // should call embedded_migrations::run_with_output.
    embedded_migrations::run_with_output(conn, &mut std::io::stdout());
}

pub fn show_posts(conn: &SqliteConnection) {
    let results = posts
        .filter(posts_published.eq(true))
        .limit(5)
        .load::<Post>(conn)
        .expect("Error loading posts");

    info!("Displaying {} posts", results.len());
    for post in results {
        info!("{}", post.title);
        info!("----------\n");
        info!("{}", post.body);
    }
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> usize {
    let new_post = NewPost { title, body };

    diesel::insert_into(posts)
        .values(&new_post)
        .execute(conn)
        .expect("Error saving new post")
}


pub fn publish_post(conn: &SqliteConnection, id: i32) {
    let num_updated = diesel::update(posts.find(id))
        .set(posts_published.eq(true))
        .execute(conn)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    let post: Post = posts
        .find(id)
        .first(conn)
        .unwrap_or_else(|_| panic!("Unable to find post {}", id));

    info!("Published post {}", post.title);
}

pub fn delete_post(conn: &SqliteConnection, pattern: String) {
    let pattern = format!("%{}%", pattern);

    let num_deleted = diesel::delete(posts.filter(posts_title.like(pattern)))
        .execute(conn)
        .expect("Error deleting posts");

    info!("Deleted {} posts", num_deleted);
}