#![allow(
    dead_code,
    unused_mut,
    unused_variables
)]
mod avec;
mod xdef;

fn setup() {
    let mut x = 0;
    xdef!(x);
    avec!();
}