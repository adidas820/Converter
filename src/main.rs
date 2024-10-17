mod prompt;

mod converter;

use prompt::*;

use converter::*;

use std::io;

use std::path::Path;

fn run_cli() -> io::Result<()> {
    println!("--- Image Converter ---");

    println!(
        "{} Enter an filepath (including filename & extension):",
        ">"
    );
    let init_filepath_raw = prompt(validate_file_exists);

    let _init_path = Path::new(&init_filepath_raw);

    println!("{} Enter directory for exported file:", ">");
    let out_dir_raw = prompt(validate_path_exists);

    let out_dir = Path::new(&out_dir_raw);

    println!("{} Enter name (with extension) of exported file:", ">");
    let out_file_raw = prompt(validate_new_file);

    let out_file = Path::new(&out_file_raw);

    let out_filepath = out_dir.join(out_file);
    let out_filepath_raw = out_filepath.to_string_lossy();

    println!("{}", "Converting...");

    let _img_result = export_file(&init_filepath_raw, &out_filepath_raw.to_string());

    println!("{} {}", "Converted! output in:", out_dir.to_string_lossy());
    Ok(())
}

fn main() {}
