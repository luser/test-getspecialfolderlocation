extern crate shell32;
extern crate winapi;

use std::ffi::OsString;
use std::ptr;
use std::os::windows::ffi::OsStringExt;
use shell32::{SHGetSpecialFolderLocation, SHGetPathFromIDListW};
use winapi::wchar_t;
use winapi::winerror::S_OK;
use winapi::shtypes::{LPITEMIDLIST, PCIDLIST_ABSOLUTE};
use winapi::minwindef::{TRUE, MAX_PATH};
use winapi::winnt::LPWSTR;

fn main() {
    let mut itemidlist: LPITEMIDLIST = ptr::null_mut();
    let res = unsafe { SHGetSpecialFolderLocation(ptr::null_mut(), winapi::shlobj::CSIDL_APPDATA, &mut itemidlist) };
    if res == S_OK {
        let mut buf: Vec<wchar_t> = vec![0; MAX_PATH];
        if unsafe { SHGetPathFromIDListW(itemidlist as PCIDLIST_ABSOLUTE, buf.as_mut_ptr() as LPWSTR) } == TRUE {
            let first = buf.split(|b| *b == 0).next().unwrap();
            let s = OsString::from_wide(first);
            println!("Got '{}'", s.to_string_lossy());
        } else {
            println!("SHGetPathFromIDListW error");
        }
    } else {
        println!("SHGetSpecialFolderLocation error: {:x}", res);
    }
}
