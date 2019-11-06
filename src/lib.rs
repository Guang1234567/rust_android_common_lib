extern crate jni;

use std::error::Error;
use std::ffi::{CStr, CString};
#[cfg(target_os = "android")]
#[allow(non_snake_case)]
use std::os::raw::c_char;

use self::jni::JNIEnv;
use self::jni::objects::{JClass, JString};
use self::jni::sys::jstring;

#[no_mangle]
pub unsafe extern fn Java_com_rust_example_android_MainActivity_greeting(env: JNIEnv, _: JClass, java_pattern: JString) -> jstring {
    let input: String =
        env.get_string(java_pattern).expect("Couldn't get java string!").into();

    // Then we have to create a new Java string to return. Again, more info
    // in the `strings` module.
    let output = env.new_string(format!("Hello, {} from Rust!", input))
        .expect("Couldn't create java string!");

    // Finally, extract the raw pointer to return.
    output.into_inner()
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
