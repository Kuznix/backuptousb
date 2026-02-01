use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use chrono::Datelike; // <-- ADD THIS LINE

fn main() {
    // Get current UTC year
    let year = chrono::Utc::now().year(); // now year() works

    // Write it to a file in OUT_DIR
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("compile_year.rs");
    let mut f = File::create(&dest_path).unwrap();
    write!(f, "pub const COMPILE_YEAR: i32 = {};", year).unwrap();
}
