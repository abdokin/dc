use std::{fs::File, process::exit, io::{Read, Write}};

#[macro_export]
macro_rules! report_error {
    ($error:expr, $file_name:expr,$row:expr, $col:expr) => {{
        eprintln!(
            "Unexpected error: {} at {}:{}:{}",
            $error, $file_name, $row, $col
        );
        std::process::exit(1);
    }};
}

pub fn read_file(file_path: &str) -> String {
    let mut file = File::open(file_path)
        .map_err(|e| {
            eprint!("Failed to open {} because {}", file_path, e);
            exit(1);
        })
        .unwrap();

    let mut content = String::new();
    file.read_to_string(&mut content)
        .expect("field to read from file");
    content
}
 pub fn save_file(file_path: &str, content: String) -> Result<(), std::io::Error> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
