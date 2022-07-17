#![feature(slice_from_ptr_range)]
#![feature(used_with_arg)]

use core::ptr;

mod str {
    use core::{slice, str};

    pub unsafe fn from_cstr_ptr<'a>(ptr: *const u8) -> &'a str {
        let start = ptr;
        let mut end = ptr;

        while end.read() != 0 {
            end = end.add(1);
        }

        str::from_utf8_unchecked(slice::from_ptr_range(start..end))
    }
}

/*const NAME: &str = "PanoramaUIClient001\0";

#[repr(C)]
pub struct Interface {
    new: unsafe extern "C" fn() -> *mut u8,
    name: *const u8,
    next: *mut Interface,
}

unsafe impl Send for Interface {}
unsafe impl Sync for Interface {}

#[no_mangle]
#[used(linker)]
pub static s_pInterfaceRegs: &Interface = &Interface {
    new: CreateInterface,
    name: NAME.as_ptr(),
    next: ptr::null_mut(),
};*/

#[repr(C)]
struct VTable {
    connect: unsafe extern "C" fn(this: *const Ui, factory: usize) -> bool,
    disconnect: unsafe extern "C" fn(this: *const Ui),
    query_interface: unsafe extern "C" fn(this: *const Ui, name: *const u8) -> *const u8,
    init: unsafe extern "C" fn(this: *const Ui) -> InitResult,
    shutdown: unsafe extern "C" fn(this: *const Ui),
    dependencies: unsafe extern "C" fn(this: *const Ui) -> *const u8,
    tier: unsafe extern "C" fn(this: *const Ui) -> i32,
    reconnect: unsafe extern "C" fn(this: *const Ui, factory: usize, name: *const u8),
    is_singleton: unsafe extern "C" fn(this: *const Ui) -> bool,
}

#[repr(C)]
pub struct Ui {
    vtable: &'static VTable,
}

impl Ui {
    pub const fn new() -> Self {
        Self {
            vtable: &VTable {
                connect,
                disconnect,
                query_interface,
                init,
                shutdown,
                dependencies,
                tier,
                reconnect,
                is_singleton,
            },
        }
    }
}

unsafe extern "C" fn connect(this: *const Ui, factory: usize) -> bool {
    println!("Ui::connect");

    true
}

unsafe extern "C" fn disconnect(this: *const Ui) {
    println!("Ui::disconnect");
}

unsafe extern "C" fn query_interface(this: *const Ui, name: *const u8) -> *const u8 {
    let name = str::from_cstr_ptr(name);

    println!("Ui::query_interface(name: {name:?})");

    ptr::null()
}

unsafe extern "C" fn init(this: *const Ui) -> InitResult {
    println!("Ui::init");

    InitResult::Ok
}

unsafe extern "C" fn shutdown(this: *const Ui) {
    println!("Ui::shutdown");
}

unsafe extern "C" fn dependencies(this: *const Ui) -> *const u8 {
    println!("Ui::dependencies");

    ptr::null()
}

unsafe extern "C" fn tier(this: *const Ui) -> i32 {
    println!("Ui::shutdown");

    0
}

unsafe extern "C" fn reconnect(this: *const Ui, factory: usize, name: *const u8) {
    println!("Ui::reconnect");
}

unsafe extern "C" fn is_singleton(this: *const Ui) -> bool {
    println!("Ui::is_singleton");

    false
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum InitResult {
    Err = 0,
    Ok = 1,
}

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum InterfaceResult {
    Ok = 0,
    Err = 1,
}

#[no_mangle]
pub unsafe extern "C" fn CreateInterface(
    name: *const u8,
    success: *mut InterfaceResult,
) -> *mut Ui {
    let name = str::from_cstr_ptr(name);

    println!("CreateInterface(name: {name:?})");

    let ui = Box::new(Ui::new());

    success.write(InterfaceResult::Ok);

    Box::leak(ui)
}
