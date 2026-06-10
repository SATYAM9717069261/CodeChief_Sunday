use std::io::{Write, stdout};
use crossterm::terminal::{self,Clear,ClearType};
use crossterm::cursor::{self,MoveTo};
use crossterm::{QueueableCommand};
use std::thread;
use crossterm::{event::poll};
use crossterm::event::{read,Event,KeyCode,KeyModifiers};
use std::time::Duration;

fn funtionality(mut width:u16,mut height:u16){

    let _ = terminal::enable_raw_mode().unwrap();
    let mut stdout = stdout();
    stdout.queue(MoveTo(width/2,height/2));
    let str:String = String::from("-");
    let mut bar = str.repeat(width as usize);
    let mut prompt = String::from("");


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
                                break;
                            }else{
                                prompt.push(x);
                            }
                        },
                        KeyCode::Enter =>{
                            chat.push(prompt.clone());
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
            stdout.queue(Clear(ClearType::All)).unwrap();

            for (row,message)  in chat.iter().enumerate(){
                stdout.queue(MoveTo(0,row as u16));
                stdout.write(message.as_bytes());
            }

            stdout.queue(MoveTo(0,height-2));
            stdout.write(bar.as_bytes());


            stdout.queue(MoveTo(0,height-1));
            stdout.write(prompt.as_bytes());


            stdout.flush();
            thread::sleep(Duration::from_millis(33));
        }
    }


}


fn main(){
    let mut stdout = stdout();
    stdout.queue(Clear(ClearType::All)).unwrap();

    if let Ok((mut width,mut height)) = terminal::size(){
        funtionality(width,height);
    }else{
        print!("Error");
    }
    println!("client");
}
