use std::{
    char,
    fs::*,
    io::prelude::*,
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
    let md_key = Mdkey {
        h1: "#".to_string(),
        h2: "##".to_string(),
        h3: "###".to_string(),
        h4: "####".to_string(),
        h5: "#####".to_string(),
        h6: "######".to_string(),
        bullet: "-".to_string(),
    };

    //? This requires the compiler to go the first closing arrow '>' and then insert after that.
    //? Another option could be the example below the code
    let html_key = HTMLkey {
        h1: "<h1></h1>".to_string(),
        h2: "<h2></h2>".to_string(),
        h3: "<h3></h3>".to_string(),
        h4: "<h4></h4>".to_string(),
        h5: "<h5></h5>".to_string(),
        h6: "<h6></h6>".to_string(),
        bullet: "<ul></ul>".to_string(),
    };
    //? EXAMPLE
    //? struct HTMLkey2{
    //?     h1_open: String,
    //?     h1_closed: String,
    //?     h2_open: String,
    //?     h2_closed: String,
    //?     h3_open: String,
    //?     h3_closed: String,
    //?     h4_open: String,
    //?     h4_closed: String,
    //?     h5_open: String,
    //?     h5_closed: String,
    //?     h6_open: String,
    //?     h6_closed: String,
    //? }
    //? let html_key = HTMLkey2 {
    //?     h1_open: "<h1>".to_string(),
    //?     h1_closed: "</h1>".to_string(),
    //?     h2_open: "<h2>".to_string(),
    //?     h2_closed: "</h2>".to_string(),
    //?     h3_open: "<h3>".to_string(),
    //?     h3_closed: "</h3>".to_string(),
    //?     h4_open: "<h4>".to_string(),
    //?     h4_closed: "</h4>".to_string(),
    //?     h5_open: "<h5>".to_string(),
    //?     h5_closed: "</h5>".to_string(),
    //?     h6_open: "<h6>".to_string(),
    //?     h6_closed: "</h6>".to_string(),
    //? };

    let mut h1_count = 0;
    let mut h2_count = 0;
    let mut h3_count = 0;
    let mut h4_count = 0;
    let mut h5_count = 0;
    let mut h6_count = 0;
    let mut bullet_count = 0;

    let md_binding = read_md("test.md");

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

        let line_wo_key = line.chars().skip(1).collect::<String>();

        //TODO Needs rework as it most likely is redundant and can be reduced. The match case is
        //TODO being handled in the 'convert_md_key_to_html_key' function anyway
        // match key.to_string() {
        //     value if value == md_key.h1 => html_key_vec.push(convert_md_key_to_html_key(value)),
        //     value if value == md_key.h2 => html_key_vec.push(convert_md_key_to_html_key(value)),
        //     value if value == md_key.h3 => html_key_vec.push(convert_md_key_to_html_key(value)),
        //     value if value == md_key.h4 => html_key_vec.push(convert_md_key_to_html_key(value)),
        //     value if value == md_key.h5 => html_key_vec.push(convert_md_key_to_html_key(value)),
        //     value if value == md_key.h6 => html_key_vec.push(convert_md_key_to_html_key(value)),
        //     value if value == md_key.bullet =>
        // html_key_vec.push(convert_md_key_to_html_key(value)),     _ => println!(""),
        // }

        html_key_vec.push(convert_md_key_to_html_key(key.to_string()));

        println!("Line: {}", line);
        println!("Key : {}", key);
        println!("Line wo Key : {}", line_wo_key);
        println!("HTML VEC : {:?} \n", html_key_vec);
    }

    //~ Prints the count of the keys
    // let output = format!(
    //     "H1: {}\nH2: {}\nH3: {}\nH4: {}\nH5: {}\nH6: {}\nBullet: {}",
    //     h1_count, h2_count, h3_count, h4_count, h5_count, h6_count, bullet_count
    // );
    // println!("{}", output);

    println!("HTML Vec : {:?}", html_key_vec)
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
