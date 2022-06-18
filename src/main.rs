use libloading::Library;
use std::{mem, ptr};

fn main() {
    let launcher = match unsafe { Library::new("bin/linux64/launcher_client.so") } {
        Ok(launcher) => launcher,
        Err(error) => panic!("failed to load the launcher: {error:?}"),
    };

    let launcher_main = match unsafe { launcher.get::<usize>(b"LauncherMain\0") } {
        Ok(launcher_main) => unsafe { launcher_main.into_raw() },
        Err(error) => panic!("failed to load the launcher entry function: {error:?}"),
    };

    let launcher_main: unsafe extern "C" fn(argc: i32, argv: *const *const u8) =
        unsafe { mem::transmute(launcher_main) };

    let args = [
        "csgo\0",
        "+fps_max\0",
        "144\0",
        "-nojoy\0",
        "-novid\0",
        "-steam\0",
    ];

    let mut args = args.iter().map(|arg| arg.as_ptr()).collect::<Vec<_>>();

    args.push(ptr::null());

    unsafe {
        (launcher_main)(args.len().saturating_sub(1) as i32, args.as_ptr());
    }
}
