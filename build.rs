fn main() {
    // cfg(target_os = "windows") seems to not work when cross-compiling from linux,
    // so we have to check if we are targeting windows like this
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        // set icon of .exe
        let mut resource = winresource::WindowsResource::new();
        resource.set_icon("res/icon/ico.ico");
        resource.compile().unwrap();
    }
}
