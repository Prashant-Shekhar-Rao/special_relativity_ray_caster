mod output;
use crate::output::output;
mod camera;
use crate::camera as cam;
mod ini_reader;
use crate::ini_reader as red;


use std::env;

fn main() {
    let Reso=red::read_lines();

    // output(vec![vec![vec![255u8,24u8,157u8];800];800]);
    output(cam::camera());
    env::set_var("RUST_BACKTRACE", "1");
    // we can perform binary search for exact point of contact. if we have a function which tells us exactly if we are inside or outside the object
}
