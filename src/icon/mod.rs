// String Offset: 0x38F4B5A

use std::{ffi::{CStr}};
use skyline::{hooks::getRegionAddress, libc::c_char};

// Replace "Item1,Item2" with your item's internal name(s)
// Do not use spaces, and insert a comma after each new entry (aside from the last)
const NEW_ENTRIES: &str = "Item1,Item2";

pub fn modify_list() -> *const c_char {
    println!("[DEBUG]: Plugin Loaded!");
    let text_base = unsafe {getRegionAddress(skyline::hooks::Region::Text) as usize};
    let str_offset = 0x38F4B5A;
    let addr = text_base + str_offset;

    let orig_ptr = addr as *const u8;
    let orig_str = unsafe {CStr::from_ptr(orig_ptr as *const i8).to_string_lossy()};
    //println!("[DEBUG] Original String:\n{}",orig_str);

    let append_str = format!("{},{}", NEW_ENTRIES, orig_str);
    skyline::patching::Patch::in_text(str_offset).cstr(&append_str);

    //let new_str = unsafe {CStr::from_ptr(orig_ptr as *const i8).to_string_lossy()};
    //println!("[DEBUG] Modified String:\n{}", new_str);

    orig_ptr
}

pub fn init() {
    modify_list();
}