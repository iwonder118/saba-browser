#![no_std]
#![no_main]

extern crate alloc;

use crate::alloc::string::ToString;
use net_wasabi::http::HttpClient;
use noli::prelude::*;

use core::fmt::Write; // for formatting
use core::result::Result::{Ok, Err};

fn main() -> u64 {
    let client = HttpClient::new();
    // Access the host machine via QEMU's user-net helper (10.0.2.2).
    // Use a high port to avoid requiring root privileges on the host side.
    match client.get("host.test".to_string(), 18080, "/test.html".to_string()) {
        Ok(res) => {
            print!("response:\n{:#?}", res);
        }
        Err(e) => {
            print!("error:\n{:#?}", e);
        }
    }
    0
}
entry_point!(main);
