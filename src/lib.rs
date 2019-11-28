#[cfg(target_os = "android")]
#[allow(non_snake_case)]
// extern crate android_log;
extern crate android_logger;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
extern crate dotenv;
extern crate jni;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;
extern crate log_panics;


use std::error::Error;
use std::time::{Duration, Instant};
use std::{sync::mpsc, thread};

use dotenv::dotenv;
use jni::JNIEnv;
use jni::objects::{GlobalRef, JClass, JObject, JString, JValue};
use jni::sys::{jbyteArray, jint, jlong, jstring};

use database::orm::do_some_db_op;
use database::sqlite::SqliteHelper;
use load_dotenv::{load_dotenv, load_dotenv_from_filename};
use my_log::MyLogger;
use chrono::{DateTime, Duration as Duration_Of_Chrono, FixedOffset, Local, Utc};
use crate::error::LibResult;


// load your .env.android file at compile time
load_dotenv_from_filename!(".env.android");


mod my_log;
mod error;
mod database;


#[no_mangle]
pub extern fn Java_com_rust_example_android_MainActivity_greeting(
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
pub extern fn Java_com_rust_example_android_MainActivity_greetingByte(
    env: JNIEnv,
    _: JClass,
    input: jbyteArray) -> jbyteArray {
    let _input = env.convert_byte_array(input).unwrap();
    let buf = [1; 2000];
    let output = env.byte_array_from_slice(&buf).unwrap();
    output
}


#[no_mangle]
pub extern fn Java_com_rust_example_android_MainActivity_rustSqlite(
    env: JNIEnv,
    _: JClass,
    database_path: JString) -> jstring {

    //dotenv().ok();

    let start = Instant::now();

    let result = MyLogger::init(env!("RUST_LOG_TAG","You forgot to export RUST_LOG_TAG"));
    match result {
        Ok(_) => {
            error!("MyLogger::init success !!!");
            warn!("MyLogger::init success !!!");
            info!("MyLogger::init success !!!");
            debug!("MyLogger::init success !!!");
            trace!("MyLogger::init success !!!");
        }
        Err(err) => error!("{}", err.description()),
    }

    let input: String =
        env.get_string(database_path).expect("Couldn't get java string!").into();

    let database_url: &str = env!("DATABASE_URL","You forgot to export DATABASE_URL path");
    let result = do_some_db_op(format!("{}_{}", input, database_url));
    match result {
        Ok(_) => info!("do_some_db_op:  success !!!!!!!!!!!!!"),
        Err(err) => error!("do_some_db_op: {}", err.description()),
    }

    let duration = start.elapsed();

    warn!("Time elapsed in expensive_function() is: {:?}", duration);

    // count logical cores this process could try to use
    let cup_nums = num_cpus::get();
    let output = env.new_string(format!("cup_nums = {} from Rust!", cup_nums))
        .expect("Couldn't create java string!");
    output.into_inner()
}

#[no_mangle]
pub extern fn Java_com_rust_example_android_MainActivity_asyncCallBack(
    env: JNIEnv,
    jclazz: JClass,
    callback: JObject) -> () {
    match asyncCallBack(env, jclazz, callback) {
        Ok(r) => r,
        Err(e) => error!("{}", e),
    }
}

fn asyncCallBack(env: JNIEnv,
                 jclazz: JClass,
                 callback: JObject) -> LibResult<()> {

    // `JNIEnv` cannot be sent across thread boundaries. To be able to use JNI
    // functions in other threads, we must first obtain the `JavaVM` interface
    // which, unlike `JNIEnv` is `Send`.
    let jvm = env.get_java_vm()?;

    // We need to obtain global reference to the `callback` object before sending
    // it to the thread, to prevent it from being collected by the GC.
    let callbackRef = env.new_global_ref(callback)?;

    // Use channel to prevent the Java program to finish before the thread
    // has chance to start.
    let (tx, rx) = mpsc::channel();

    let _ = thread::Builder::new()
        .name("`Thread in rust`".into())
        .spawn(move || {
            info!("I am a thread in rust, my id = {:?}", thread::current().id());

            // Signal that the thread has started.
            tx.send(()).unwrap();

            // Use the `JavaVM` interface to attach a `JNIEnv` to the current thread.
            let env = jvm.attach_current_thread().unwrap();

            // Then use the `callback` with this newly obtained `JNIEnv`.
            let callback = callbackRef.as_obj();

            thread::sleep(Duration::from_millis(100));

             warn!("Before call ....................");
            // Now we can use all available `JNIEnv` functionality normally.
            let ret = env.call_method(callback, "onCallBack", "()Ljava/lang/String;", &[])
                .unwrap();


            let ouput_from_cb: String = match ret {
                JValue::Object(v) => env.get_string(JString::from(v)).unwrap().into(),
                _ => String::from(""),
            };

            warn!("After call ......................{:?}", ouput_from_cb);

            // Warn: The current thread is detached automatically when `env` goes out of scope.
            // Should: https://docs.rs/jni/0.14.0/jni/objects/struct.GlobalRef.html
            /*
            It is recommended that a native thread that drops the global reference is attached to the Java thread (i.e., has an instance of JNIEnv). If the native thread is not attached, the GlobalRef#drop will print a warning and implicitly attach and detach it, which significantly affects performance.
            */
            drop(callbackRef)
        }
        );

    // Wait until the thread has started.
    rx.recv()?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
