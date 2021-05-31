use std::io::Write;

pub fn install() {
    // Only use custom panic handler if we are in release mode:
    #[cfg(not(debug_assertions))]
    std::panic::set_hook(Box::new(|panic_info: &core::panic::PanicInfo| {
        // get info:
        let (file, line) = if let Some(loc) = panic_info.location() {
            (loc.file(), loc.line())
        } else {
            ("", 0)
        };
        let info = panic_info.payload().downcast_ref::<&str>().unwrap_or(&"");

        // print to stdout:
        println!(
            "System panic occurred in file '{}' at line {}! Message: {:?}",
            file, line, info
        );
        let _ = std::io::stdout().flush();

        // create message box:
        let _ = msgbox::create(
            "Engine System Panic",
            &format!(
                "System panic occurred in file '{}' at line {}! Message: {:?}",
                file, line, info
            ),
            msgbox::IconType::Error,
        );
    }));
}
