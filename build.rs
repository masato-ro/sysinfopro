// build.rs
fn main() {
    // 這會在編譯時期告訴 Slint 編譯器使用哪種風格
    slint_build::compile_with_config(
        "ui/main.slint",
        slint_build::CompilerConfiguration::default()
            .with_style("cupertino".into())
    ).unwrap();
}