pub mod cli;

use std::{
    fs::*,
    io::prelude::*,
    path::Path,
};

struct Mdkey {
    h1: String,
    h2: String,
    h3: String,
    h4: String,
    h5: String,
    h6: String,
    bullet: String,
}

struct HTMLkey {
    h1: String,
    h2: String,
    h3: String,
    h4: String,
    h5: String,
    h6: String,
    bullet: String,
}

fn main() {
    let (md_file_cli, html_file_cli) = cli::cli();

    //~ Allows user to not have to include file extension through CLI
    let md_file_cli = format!("{}.md", md_file_cli);
    let md_binding = read_md(&md_file_cli);

    //~ Split by line. Allows us to handle each key for each line
    let md_vec: Vec<&str> = md_binding.trim().split("\n").collect();

    //~ Iterate over the lines in the vec and check what the first character is
    //& If its a valid key => Iterate its count variable
    //& If its an escape character => Go to the next valid character. If that dont exist go to next
    //& line If its neither and just normal text => //TODO

    let mut html_key_vec: Vec<String> = vec![];

    for line in md_vec {
        //TODO Handle the match case better.
        let key = match line.chars().nth(0) {
            Some(key) => key,
            None => 'c',
        };

        //~ Used in the iterator below to remove the key based on the length of the key
        //~ If the line is : `# Hello World` it gets the length of '#' (which is 1) and removes that many characters from the start
        //~ If the line is : `## Hello World` it gets the length of '##' (which is 2)
        let md_key_length = key.to_string().len();

        //~ Removes the key from the line and returns the rest
        let line_wo_key = line
            .chars()
            .skip(md_key_length)
            .collect::<String>();
        
        //~ Takes the markdown key and returns the corresponding html key
        let html_key = convert_md_key_to_html_key(key.to_string());

        //~ Inserts the line_wo_key after the first '>' in the html_key
        //~ Works because the `find()` function locates the FIRST instance of the character stopping the line_wo_key being added to any other instance of the '>'
        let mut html = String::new();
        if let Some(position) = html_key.find('>') {
            let (opening_tag, closing_tag) = html_key.split_at(position + 1);
            html = format!("{}{}{}", opening_tag, line_wo_key, closing_tag);

        } //TODO if it cant find the '>' something should probably happen
        // else {
        //     println!("No '>' character found in the string.");
        // }

        html_key_vec.push(html);

        // println!("Line: {}", line);
        // println!("Key : {}", key);
        // println!("Line wo Key : {}", line_wo_key);
    }

    let mut html = String::new();
    for i in html_key_vec {
        html += &i;
    }

    create_html(&html_file_cli, html);
    
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

fn convert_md_key_to_html_key(md_key_value: String) -> String {
    let md_key = Mdkey {
        h1: "#".to_string(),
        h2: "##".to_string(),
        h3: "###".to_string(),
        h4: "####".to_string(),
        h5: "#####".to_string(),
        h6: "######".to_string(),
        bullet: "-".to_string(),
    };

    let html_key = HTMLkey {
        h1: "<h1></h1>".to_string(),
        h2: "<h2></h2>".to_string(),
        h3: "<h3></h3>".to_string(),
        h4: "<h4></h4>".to_string(),
        h5: "<h5></h5>".to_string(),
        h6: "<h6></h6>".to_string(),
        bullet: "<ul></ul>".to_string(),
    };

    match md_key_value.to_string() {
        value if value == md_key.h1 => return html_key.h1,
        value if value == md_key.h2 => return html_key.h2,
        value if value == md_key.h3 => return html_key.h3,
        value if value == md_key.h4 => return html_key.h4,
        value if value == md_key.h5 => return html_key.h5,
        value if value == md_key.h6 => return html_key.h6,
        value if value == md_key.bullet => return html_key.bullet,
        _ => format!("Key Not found"), /* TODO this should just measure/test the text itself and
                                        * no other key so we
                                        * dont need to to add it to the html_key_vec so it should
                                        * be rendered in the <p> tag */
    }
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
