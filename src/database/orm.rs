use std::env;
use std::io::Write;

use diesel::connection::SimpleConnection;
use diesel::prelude::*;
use diesel::SqliteConnection;

use crate::error::LibResult;
use crate::my_log::MyLogger;

use super::models::{NewPost, Post};
use super::schema::posts::dsl::{posts as posts_table, published as posts_published, title as posts_title};

pub fn do_some_db_op(database_url: String) -> LibResult<()> {
    let conn = &establish_connection(database_url)?;

    db_migrations(conn)?;

    let inserted_rows = create_post(conn, "title001", "body001")?;

    publish_post(conn, 1)?;

    show_posts(conn)?;

    Ok(())
}

pub fn establish_connection(database_url: String) -> LibResult<SqliteConnection> {
    /*
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    */
    Ok(SqliteConnection::establish(&database_url)?)
}

pub fn db_migrations(conn: &SqliteConnection) -> LibResult<()> {
    warn!(">>>>>>>---------------------  db_migrations  ---------------------");
    embed_migrations!("./migrations");
    let output = &mut MyLogger::new();
    let r = embedded_migrations::run_with_output(conn, output)?;
    output.flush();
    warn!("<<<<<<<---------------------  db_migrations  ---------------------");
    Ok(r)
}

pub fn show_posts(conn: &SqliteConnection) -> LibResult<Vec<Post>> {
    let posts: Vec<Post> = posts_table
        .filter(posts_published.eq(true))
        .limit(5)
        .load::<Post>(conn)?;
    //.expect("Error loading posts");

    error!("Displaying {} posts", posts.len());
    warn!("Displaying {} posts", posts.len());
    info!("Displaying {} posts", posts.len());
    debug!("Displaying {} posts", posts.len());
    trace!("Displaying {} posts", posts.len());
    for post in &posts {
        info!("{}", post.title);
        info!("----------\n");
        info!("{}", post.body);
    }

    Ok(posts)
}

pub fn create_post(conn: &SqliteConnection, title: &str, body: &str) -> LibResult<usize> {
    let new_post = NewPost { title, body };

    let inserted_rows = diesel::insert_into(posts_table)
        .values(&new_post)
        .execute(conn)?;
    //.expect("Error saving new post")

    info!("create_post  inserted_rows={}", inserted_rows);

    Ok(inserted_rows)
}

pub fn publish_post(conn: &SqliteConnection, id: i32) -> LibResult<Post> {
    let updated_rows = diesel::update(posts_table.find(id))
        .set(posts_published.eq(true))
        .execute(conn)?;
    //.unwrap_or_else(|_| panic!("Unable to find post {}", id));

    let post: Post = posts_table
        .find(id)
        .first(conn)?;
    //.unwrap_or_else(|_| panic!("Unable to find post {}", id));

    info!("Published post {}", post.title);

    Ok(post)
}

pub fn delete_post(conn: &SqliteConnection, pattern: String) -> LibResult<usize> {
    let pattern = format!("%{}%", pattern);

    let deleted_rows = diesel::delete(posts_table.filter(posts_title.like(pattern)))
        .execute(conn)?;
    //.expect("Error deleting posts");

    info!("Deleted {} posts", deleted_rows);

    Ok(deleted_rows)
}