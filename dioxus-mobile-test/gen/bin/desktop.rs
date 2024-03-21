
fn main() {
    #[cfg(not(any(target_os = "android", target_os = "ios")))]
    diouxs_mobile_test::main();
}
