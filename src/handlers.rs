use std::{
    fs::{self, read_dir, remove_file, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
};

fn store_template(file_to_template: &File, store_at: &PathBuf) -> Result<(), std::io::Error> {
    let mut file_to_write_to = File::create(store_at)?;

    let buffered = BufReader::new(file_to_template);

    for line in buffered.lines() {
        if let Ok(ok_line) = line {
            writeln!(file_to_write_to, "{}", ok_line)?;
        }
    }

    Ok(())
}

fn get_template_location(template_name: &String) -> Result<PathBuf, std::io::Error> {
    let template_directory = PathBuf::from("./templates/");

    if !template_directory.exists() {
        fs::create_dir(&template_directory)?;
    }

    Ok(PathBuf::from(format!("./templates/{}", template_name)))
}

pub fn list_templates() -> Result<(), std::io::Error> {
    let no_file = String::from("");
    let dir_path = get_template_location(&no_file)?;

    let dirs = read_dir(dir_path)?;

    println!("");
    println!("The following templates exist:");

    for dir in dirs {
        if let Ok(d) = dir {
            println!("* {:?}", d.file_name())
        }
    }

    println!("");

    Ok(())
}

pub fn update_template(
    template_name: &String,
    path_to_template: &PathBuf,
) -> Result<(), std::io::Error> {
    let file_to_read_from = File::open(path_to_template)?;

    let file_location = get_template_location(template_name)?;

    if file_location.exists() {
        store_template(&file_to_read_from, &file_location)?;
    } else {
        println!("Template '{}' does not exist yet.", template_name);
        println!("If you want to add it, use the add command.");
    }
    Ok(())
}

pub fn add_template(
    template_name: &String,
    path_to_template: &PathBuf,
) -> Result<(), std::io::Error> {
    let file_to_read_from = File::open(path_to_template)?;

    let file_location = get_template_location(template_name)?;

    if !file_location.exists() {
        store_template(&file_to_read_from, &file_location)?;
    } else {
        println!("Template '{}' was not created. It already exists.", template_name);
        println!("If you want to change it, use the update command.");
    }
    Ok(())
}

pub fn remove_template(template_name: &String,) -> Result<(), std::io::Error> {
    let file_location = get_template_location(template_name)?;
    remove_file(file_location)?;

    println!("Template with key {} deleted.", &template_name);
    Ok(())
}