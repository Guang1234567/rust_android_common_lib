extern crate android_log;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate jni;
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
#[macro_use]
extern crate log;
extern crate log_panics;
#[macro_use]
extern crate lazy_static;


use std::error::Error;

use dotenv::dotenv;
use jni::JNIEnv;
use jni::objects::{JClass, JString};
use jni::sys::jstring;

use database::orm::do_some_db_op;
use database::sqlite::SqliteHelper;
use logger::MyLogger;

mod logger;
mod error;
mod database;

#[no_mangle]
pub unsafe extern fn Java_com_rust_example_android_MainActivity_greeting(
    env: JNIEnv,
    _: JClass,
    java_pattern: JString) -> jstring {
    /*
    android_log::init("app_rust").unwrap();
    trace!("Initialized Rust");
    debug!("Address is {:p}", Java_com_rust_example_android_MainActivity_greeting as *const ());
    info!("Did you know? {} = {}", "1 + 1", 2);
    warn!("Don't log sensitive information!");
    error!("Nothing more to say");
    */


    let input: String =
        env.get_string(java_pattern).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("Hello, {} from Rust!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}

#[no_mangle]
pub unsafe extern fn Java_com_rust_example_android_MainActivity_rustSqlite(
    env: JNIEnv,
    _: JClass,
    database_path: JString) -> jstring {

    dotenv().ok();

    let result = MyLogger::init("app_rust_sql_123");
    match result {
        Ok(_) => info!("MyLogger::init success !!!!!!!!!!!!!"),
        Err(err) => error!("{}", err.description()),
    }

    let input: String =
        env.get_string(database_path).expect("Couldn't get java string!").into();

    let result = do_some_db_op(input.clone());
    match result {
        Ok(_) => info!("do_some_db_op:  success !!!!!!!!!!!!!"),
        Err(err) => error!("do_some_db_op: {}", err.description()),
    }

    let output = env.new_string(format!("Hello, {} from Rust!", input))
        .expect("Couldn't create java string!");
    output.into_inner()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
