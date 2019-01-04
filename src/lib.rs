use std::io;
mod quick;
use crate::quick::quicksort;
mod selection;
use crate::selection::selcsort;
mod bucket;
use crate::bucket::bucketsort;
use jni::JNIEnv;
use jni::objects::{JClass,JObject,JValue};
use jni::sys::{jint,jintArray,jsize,_jobject,jarray,jobject,jclass};

#[no_mangle]
pub extern "system" fn Java_com_sample_test_Library_Initiator(
    env: JNIEnv,
    _class: jclass,
    array: jintArray,
    c: jint) {

    let mut x = env.get_array_length(array).unwrap() as usize;

    let mut buf: Vec<i32> = vec![0;x];

    env.get_int_array_region(array,0, buf.as_mut_slice()).expect("error at getting array");

    println!("Original:{:?}",&buf);

    match c {
        1 => {&quicksort(buf.as_mut_slice()); println!("Sorted:{:?}",&buf);},
        2 => {&selcsort(buf.as_mut_slice()); println!("Sorted:{:?}",&buf);},
        3 => println!("Sorted:{:?}",&bucketsort(buf,|int| int/10)),
        _ => println!("Invalid Choice"),
    }

}
