#![doc = include_str!(concat!("../", env!("CARGO_PKG_README")))]
#![no_std]
#[cfg(feature = "std")]
extern crate std;
#[cfg(feature = "c-exports")]
pub mod exports;
pub mod types;
pub mod reader;
pub mod writer;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

