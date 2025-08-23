#![deny(clippy::all)]

use napi_derive::napi;

#[napi]
pub fn plus(input: u32) -> u32 {
    input + 100
}
