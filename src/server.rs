use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::{Write,Read};
use std::fmt;
use std::result;
use std::sync::mpsc::{Sender,Receiver,channel};


const ADDRESS:&str ="127.0.0.1:6969";
const SAFE_MODE:bool = true;

type Result<T> = result::Result<T,()>;

struct Sensitive<T>{
    inner:T
}

impl<T>  Sensitive<T>{
    fn new(inner:T) -> Self{
        Self{inner}
    }
}

impl<T: fmt::Display> fmt::Display for Sensitive<T>{
    fn fmt(&self, f:&mut fmt::Formatter<'_>) -> fmt::Result{
        if SAFE_MODE{
            writeln!(f,"[REDACTED]")
        }else{
            writeln!(f,"{inner}",inner = self.inner)
        }
    }
}

enum Message{
    ClientConnected,
    DisConnected,
    NewMessage(Vec<u8>),
}


fn client(mut stream: TcpStream, message: Sender<Message>) -> Result<()>{
    message.send(Message::ClientConnected).map_err(|err| {
        eprintln!("couldn't send message to server thread {:?}",err)
    })?;
 // let _ = writeln!(stream,"Working").map_err(|err| {
 //     eprint!("ERROR: can;t write message : {:?}",err);
 // });
    let mut buffer = Vec::new();
    buffer.resize(64,0);
    loop{
        let n = stream.read(&mut buffer).map_err(|err| {
            eprintln!("Couldn't read message : {err}");
            let _ = message.send(Message::DisConnected);
        })?;
        message.send(Message::NewMessage(buffer[0..n].to_vec())).map_err(|err| {
            eprintln!("couldn't send message to server thread {err}")
        })?;
    };

}

fn server(message: Receiver<Message>)->Result<()>{
    todo!();
}

fn main() -> Result<()>{
    let listener = TcpListener::bind(ADDRESS).map_err(|err| {
        eprintln!("couldn't bind {ADDRESS} : {:?}",err);
    })?;

    let (message_sender, message_reciver) = channel();
    thread::spawn(|| server(message_reciver));

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                let message_sender = message_sender.clone();
                thread::spawn(|| client(stream,message_sender));
            },
            Err(err) => eprint!("Can't Accept Conection {:?}", err)
        };
    }
    Ok(())
}
