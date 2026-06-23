use std::io::{Write, stdout, ErrorKind, Read};
use crossterm::terminal::{self,Clear,ClearType};
use crossterm::cursor::{self,MoveTo};
use crossterm::{QueueableCommand};
use std::thread;
use crossterm::{event::poll};
use crossterm::event::{read,Event,KeyCode,KeyModifiers};
use std::time::Duration;
use std::cmp::max;

use std::net::TcpStream;

struct Rect{ x:u16, y:u16, h:u16, w:u16 }

fn chat_window( stdout: &mut impl Write, chat: &Vec<String>, boundary: Rect) {
    let n = chat.len();
    let m = max(0, n  as i32 - boundary.h as i32) as u16;

    for (row,line) in chat.iter().skip(m as usize).enumerate(){
        stdout.queue(MoveTo(boundary.x, boundary.y + row as u16));
        let bytes = line.as_bytes();
        stdout.write(bytes.get(0..boundary.w as usize).unwrap_or(bytes));
    }
}

fn funtionality(mut width:u16,mut height:u16){

    let _ = terminal::enable_raw_mode().unwrap();
    let mut stdout = stdout();
    stdout.queue(MoveTo(width/2,height/2));
    let str:String = String::from("-");
    let mut bar = str.repeat(width as usize);
    let mut prompt = String::from("");

    let mut stream = TcpStream::connect("127.0.0.1:6969").unwrap();
    let _ =  stream.set_nonblocking(true).unwrap();
    let mut buf = [0;64];

    let mut chat = Vec::new();

    while let Ok(avail) = poll(Duration::from_secs(0)){
        if avail == true{
            match read(){
                Ok(Event::Resize(nw,nh)) =>{
                    width  = nw;
                    height = nh;
                    bar = str.repeat(width as usize);
                },
                Ok(Event::Key(event)) => {

                    match event.code{
                        KeyCode::Char(x) => {
                            if x == 'c' && event.modifiers.contains(KeyModifiers::CONTROL){
                                terminal::disable_raw_mode().unwrap();
                                break;
                            }else{
                                prompt.push(x);
                            }
                        },
                        KeyCode::Enter =>{
                            chat.push(prompt.clone());
                            stream.write(&prompt.as_bytes()).unwrap();
                            prompt.clear();
                        },
                        _ => {
                        }
                    }



                },
                _ =>{
                }

            }
        }else{
            // no event found
            match stream.read(&mut buf) {
                Ok(n) => {
                    chat.push(
                        std::str::from_utf8(&buf[..n])
                        .unwrap()
                        .to_string()
                    );
                }
                Err(err) => {
                    if err.kind() != std::io::ErrorKind::WouldBlock {
                        panic!("{err}");
                    }
                }
            }

            stdout.queue(Clear(ClearType::All)).unwrap();
            chat_window(&mut stdout,&chat,Rect{ x:0, y:0, w:width, h:height-2 });

            stdout.queue(MoveTo(0,height-2));
            stdout.write(bar.as_bytes());


            stdout.queue(MoveTo(0,height-1));
            {
                let bytes = prompt.as_bytes();
                stdout.write(bytes.get(0..width as usize).unwrap_or(bytes));
            }


            stdout.flush();
            thread::sleep(Duration::from_millis(33));
        }
    }

    terminal::disable_raw_mode().unwrap();

}


fn main(){
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All)).unwrap();

    if let Ok((mut width,mut height)) = terminal::size(){
        funtionality(width,height);
    }else{
        print!("Error");
    }
    stdout.flush().unwrap();
    println!("client");
}
