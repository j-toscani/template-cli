use std::{
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

pub fn add_template(
    template_name: &String,
    path_to_template: &PathBuf,
) -> Result<(), std::io::Error> {
    let file_to_read_from = File::open(path_to_template)?;
    let template_directory = PathBuf::from("./templates/");

    if !template_directory.exists() {
        fs::create_dir(&template_directory)?;
    }

    let file_location = PathBuf::from(format!("./templates/{}", template_name));

    if !file_location.exists() {
        let mut file_to_write_to = File::create(file_location)?;

        let buffered = BufReader::new(file_to_read_from);

        for line in buffered.lines() {
            if let Ok(ok_line) = line {
                writeln!(file_to_write_to, "{}", ok_line)?;
            }
        }
    } else {
        println!("Template was not created. It already exists.");
        println!("If you want to change it, use the update command.");
    }
    Ok(())
}
