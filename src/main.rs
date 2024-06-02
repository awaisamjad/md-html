use std::{
    char,
    fs::*,
    io::prelude::*,
};

struct MD_KEY {
    h1: String,
    h2: String,
    h3: String,
    h4: String,
    h5: String,
    h6: String,
    bullet: String,
}

fn main() {
    let md_key = MD_KEY {
        h1: "#".to_string(),
        h2: "##".to_string(),
        h3: "###".to_string(),
        h4: "####".to_string(),
        h5: "#####".to_string(),
        h6: "######".to_string(),
        bullet: "-".to_string(),
    };

    let mut h1_count = 0;
    let mut h2_count = 0;
    let mut h3_count = 0;
    let mut h4_count = 0;
    let mut h5_count = 0;
    let mut h6_count = 0;
    let mut bullet_count = 0;

    let md_binding = read_md("test.md");
    let md_vec: Vec<&str> = md_binding.split_whitespace().collect();

    for i in md_vec {
        match i.to_string() {
            value if value == md_key.h1 => h1_count += 1,
            value if value == md_key.h2 => h2_count += 1,
            value if value == md_key.h3 => h3_count += 1,
            value if value == md_key.h4 => h4_count += 1,
            value if value == md_key.h5 => h5_count += 1,
            value if value == md_key.h6 => h6_count += 1,
            value if value == md_key.bullet => bullet_count += 1,
            _ => println!(""),
        }
    }
    let output = format!(
        "H1: {}\nH2: {}\nH3: {}\nH4: {}\nH5: {}\nH6: {}\nBullet: {}",
        h1_count, h2_count, h3_count, h4_count, h5_count, h6_count, bullet_count
    );
    println!("{}", output);
}

fn read_md(filepath: &str) -> String {
    //~ Open the file
    let mut file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error Opening File: {}", error);
            panic!()
        }
    };
    //~ Put into String buffer
    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error Reading File to String Buffer: {}", error);
            panic!()
        }
    };

    file_string
}

fn create_html(filepath: &str, filename: &str, contents: String) {
    let file = format!("{}/{}.html", filepath, filename);
    let html = match File::create(file) {
        Ok(html) => html,
        Err(error) => {
            eprintln!("Error Creating File: {}", error);
            panic!()
        }
    };
    //TODO Write to file
}
