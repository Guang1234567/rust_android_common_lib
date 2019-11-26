use std::io::{Result as IoResult, Write};
use crate::error::LibResult;
//use android_log::AndroidLogger;
use log::Level;
use android_logger::{Config,FilterBuilder};

/*
lazy_static! {
    static ref MY_ANDROID_LOGGER: AndroidLogger = AndroidLogger::new("rust_demo_so");
}
*/

pub struct MyLogger {}

impl MyLogger {
    pub fn init<S: Into<String>>(tag: S) -> LibResult<()> {
        //MY_ANDROID_LOGGER.init()?;
        android_logger::init_once(
            Config::default()
                .with_min_level(Level::Trace) // limit log level
                /*.with_tag("rust_demo_so") // logs will show under mytag tag
                .with_filter( // configure messages for specific crate
                              FilterBuilder::new()
                                  .parse("debug,greetings=info") // `greetings` is dynamic-share-lib-name in Cargo.toml
                                  .build())*/
        );

        log_panics::init();
        Ok(())
    }

    pub fn new() -> Self {
        Self {}
    }
}

impl Write for MyLogger {
    fn write(&mut self, buf: &[u8]) -> IoResult<usize> {
        info!("{}", String::from_utf8_lossy(buf));
        Ok(buf.len())
    }

    fn flush(&mut self) -> IoResult<()> {
        log::logger().flush();
        Ok(())
    }
}