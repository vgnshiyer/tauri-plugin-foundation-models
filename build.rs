const COMMANDS: &[&str] = &["check_availability"];

fn main() {
    // https://github.com/tauri-apps/tauri/issues/11103
    #[cfg(any(target_os = "ios", target_os = "macos"))]
    std::env::remove_var("SDKROOT");

    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .ios_path("ios")
        .build();

    #[cfg(any(target_os = "ios", target_os = "macos"))]
    {
        use swift_rs::SwiftLinker;

        SwiftLinker::new("15")
            .with_ios("18")
            .with_package("tauri-plugin-foundation-models", "./ios")
            .link();
    }
}
