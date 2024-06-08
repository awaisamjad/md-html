use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    //Enter file path for Markdown file (Include)
    #[arg(short = 'm', long = "markdown")]
    md_file_path: String,

    #[arg(long = "html", default_value = "index.html")]
    html_file_path: String,

    #[arg(short, long)]
    port: Option<String>,
}

pub fn init() -> (String, String) {
    let args = Args::parse();

    return (args.md_file_path, args.html_file_path);
}

// mdh src/md_file.md -o src/html_file.html -p 3000
