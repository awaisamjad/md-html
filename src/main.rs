mod cli;
mod html;
mod md;

use std::{
    fs::File,
    io::{
        Read,
        Write,
    },
    path::Path,
};

use md::KeysWithSpace;

fn main() {
    let (md_file_cli, html_file_cli) = cli::init();

    //~ Allows user to not have to include file extension through CLI
    let md_file_cli = format!("{}.md", md_file_cli);
    let md_binding = read_md(&md_file_cli);

    //~ Split by line. Allows us to handle each key for each line
    let md_vec: Vec<&str> = md_binding.trim().split("\n").collect();

    //& HANDLING THE KEYS THAT NEED A SPACE
    //TODO 1 Put in separate function.
    //TODO 2 As we know which keys need a space we can therefore handle all the ones that dont
    //~ Checks the characters over each line in the vec
    //~ It joins the characters to an empty temp string until it sees a space
    //~ This allows us to handle the keys that need a space
    for line in md_vec {
        let mut temp = String::new();
        for character in line.to_string().chars() {
            if character == ' ' {
                break;
            }
            temp.push(character);
        }

        let md_key_with_space: KeysWithSpace = KeysWithSpace {
            h1: "#".to_string(),
            h2: "##".to_string(),
            h3: "###".to_string(),
            h4: "####".to_string(),
            h5: "#####".to_string(),
            h6: "######".to_string(),
            unordered_list: "-".to_string(),
        };

        // match temp {
        //     value if value == 
        // }

        println!("Temp : {}", temp);
    }

    // println!("{:?}", md_vec);
}

fn read_md(filepath: &str) -> String {
    //~ Open the file
    //TODO Handle the match properly
    let mut file = match File::open(filepath) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error Opening File: {}", error);
            panic!()
        }
    };
    //~ Put into String buffer
    //TODO Handle the match properly
    //TODO Remove return characters '\r'
    let mut file_string = String::new();
    match file.read_to_string(&mut file_string) {
        Ok(file) => file,
        Err(error) => {
            eprintln!("Error Reading File to String Buffer: {}", error);
            panic!()
        }
    };

    //~ Remove the return characters '\r'
    let file_string = file_string.replace("\r", "to");

    file_string
}

fn create_html(filepath: &str, contents: String) {
    let html_front_template = "
        <!DOCTYPE html>
    <html lang=\"en\">
    <head>
        <meta charset=\"UTF-8\">
        <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">
        <title>Document</title>
    </head>
    <body>";

    let html_back_template = "
    </body>
    </html>";

    let html = html_front_template.to_owned() + &contents + html_back_template;
    // println!("HTML : {:?}", html);

    // Ensure the directory exists
    let path = Path::new(filepath);
    if !path.exists() {
        std::fs::create_dir_all(path).expect("Failed to create directories");
    }

    let file_path = format!("{}.html", filepath);
    let mut file = File::create(&file_path).expect("Failed to create file");

    file.write_all(html.as_bytes())
        .expect("Failed to write to file");
    println!("HTML file created at: {}", file_path);
}
