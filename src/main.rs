use std::env;
use std::fs;
use windows_sys::Win32::Storage::FileSystem::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let name = args.get(1).expect("Expect an argument.");

    fs::write(
        "desktop.ini",
        format!("[.ShellClassInfo]\nLocalizedResourceName={}\n", name),
    )
    .expect("Error");

    unsafe {
        SetFileAttributesA(
            b"desktop.ini\0".as_ptr(),
            FILE_ATTRIBUTE_ARCHIVE | FILE_ATTRIBUTE_HIDDEN | FILE_ATTRIBUTE_SYSTEM,
        );
    }
}
