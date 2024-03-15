
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    // let (location, line, column) = match info.location() {
    //     Some(loc) => (loc.file(), loc.line(), loc.column()),
    //     _ => ("???", 0, 0),
    // };

    crate::println!(
        "Kernel panic!\n\n\
        {:?}",
        info,
    );

    crate::_park();
}