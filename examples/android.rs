#[path = "test.rs"]
mod test;


#[cfg(target_os = "android")]
#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
fn main() {
    let _trace;
    if ndk::trace::is_trace_enabled() {
        _trace = ndk::trace::Section::new("Surreal on Android").unwrap();
    }
    println!("Hello from Surreal");

    test::main();
}

#[allow(unused)]
#[cfg(not(target_os = "android"))]
fn main() {
    panic!("Not android (use `cargo apk build --example android`)");
}