#![allow(non_camel_case_types)]

mod bindgen {
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
}
pub use bindgen::*;
