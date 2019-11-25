use std::io::{Result as IoResult, Write};
use crate::error::LibResult;
use android_log::AndroidLogger;

lazy_static! {
    static ref MY_ANDROID_LOGGER: AndroidLogger = AndroidLogger::new("lhg123");
}

pub struct MyLogger {}

impl MyLogger {
    pub fn init<S: Into<String>>(tag: S) -> LibResult<()> {
        MY_ANDROID_LOGGER.init()?;
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
        //log::logger().flush();
        Ok(())
    }
}