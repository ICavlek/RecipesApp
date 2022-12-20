mod constants;
use constants::*;
mod recipe;
use recipe::*;
mod messages;
use messages::*;

/// Main function
fn main() {
    let dinamo: i32 = 1;
    println!("{}", STORAGE_FILE_PATH);
}
