fn main() {
    // 在 tests7 中，我们需要设置一个名为 TEST_FOO 的环境变量
    // 并打印到标准输出以让 Cargo 设置它
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs(); // 使用当前时间戳来设置环境变量
    println!("cargo:rustc-env=TEST_FOO={}", timestamp);

    // 在 tests8 中，我们需要启用 "pass" 特性
    // 使用 println! 告诉 Cargo 启用这个特性
    println!("cargo:rustc-cfg=feature=\"pass\"");
}
