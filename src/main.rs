use std::fs::File;
use std::process;
use std::io::{self, BufRead, BufReader, Write};
use std::env;

fn escape_html(text: &str) -> String {
    text
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
}

fn parse_inline(text: &str) -> String {

    let mut text = text.to_string();

    let mut parts:Vec<String> = text.split("**").map(String::from).collect();
    for i in 0..parts.len(){
        if i%2 == 1{
            parts[i] = format!("<strong>{}</strong>",parts[i]);
        }
    }
    text = parts.join("");

    let mut parts:Vec<String> = text.split("*").map(String::from).collect();
    for i in 0..parts.len(){
        if i%2 == 1{
            parts[i] = format!("<em>{}</em>",parts[i]);
        }
    }
    text = parts.join("");

    let mut parts:Vec<String> = text.split("`").map(String::from).collect();
    for i in 0..parts.len(){
        if i%2 == 1{
            parts[i] = format!("<Code>{}</Code>",parts[i]);
        }
    }
    text = parts.join("");

    return text.to_string();
}

fn parse_markdown<T: BufRead>(reader:T ,file:&mut File ){
    let mut in_list = false;

    for line in reader.lines(){
        let line  = line.unwrap();
        let line = escape_html(line.trim());

        if line.is_empty(){
            if in_list{
                writeln!(file,"</ul>").unwrap();
                in_list = true;
            }
            continue;
        }

        if line.starts_with("- "){
            if in_list == false {
                writeln!(file,"<ul>").unwrap();
                in_list = true;
            }
            let content  = &line[2..];
            let content  = parse_inline(&content);
            writeln!(file, "<li>{}</li>", content).unwrap();
            continue;
        }

        if line.starts_with("#"){
            let count_hash = line.chars().take_while(|&c| c == '#').count();
            if count_hash>= 1 && count_hash<= 6{
                let content = line[count_hash+1..].trim();
                let content  = parse_inline(&content);
                writeln!(file, "<h{}>{}</h{}>", count_hash,content,count_hash).unwrap();
            }
            continue;
        }

        if in_list {
            writeln!(file, "</ul>").unwrap();
            in_list = false;
        }
        let content = parse_inline(&line);
        writeln!(file, "<p>{}</p>", content).unwrap();    }
}


fn main()->io::Result<()>{
    println!("=== Markdown to HTML Generator ===");

    let path = match env::current_dir(){
        Ok(path) => path,
        Err(_) => {
            println!("Error on Path Reading");
            process::exit(1);
        }
    };

    let file_path = path.join("src").join("input.md");
    let file = match File::open(file_path){
        Ok(file)=>file,
        Err(_) => {
            println!("Error on Opening");
            process::exit(1);
        },
    };

    let file_out_path = path.join("src").join("output.html");
    let mut output = File::create(file_out_path)?;
    let reader = BufReader::new(file);
    parse_markdown(reader,&mut output);
    Ok(())
}
