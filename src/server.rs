use std::net::{TcpListener, TcpStream, IpAddr, SocketAddr, Shutdown};
use std::thread;
use std::io::{Write,Read};
use std::fmt;
use std::result;
use std::sync::mpsc::{Sender,Receiver,channel};
use std::sync::Arc;
use std::collections::HashMap;
use std::time::{SystemTime, Duration};

const ADDRESS:&str ="127.0.0.1:6969";
const SAFE_MODE:bool = true;
const BAN_LIMIT:Duration = Duration::from_secs(10*60);
const MESSAGE_RATE:Duration = Duration::from_secs(1);
const STRIKE_COUNT:usize = 10;

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
    ClientConnected{
        author:Arc<TcpStream>
    },
    ClientDisConnected{
        author_add:SocketAddr,
    },
    NewMessage{
        author_add:SocketAddr,
        bytes:Vec<u8>
    },
}

struct Client{
    conn: Arc<TcpStream>,
    last_message: SystemTime,
    strike_count:usize
}

fn client(mut stream: Arc<TcpStream>, message: Sender<Message>) -> Result<()>{
    let author_add = stream.peer_addr().map_err(|err| {
        eprintln!("Couldn't get Peer Address {err}");
    })?;

    message.send(Message::ClientConnected{
        author: stream.clone()
    }).map_err(|err| {
        eprintln!("couldn't send message to server thread {:?}",err)
    })?;

    let mut buffer = Vec::new();
    buffer.resize(64,0);
    loop{

        let n = stream.as_ref().read(&mut buffer).map_err(|err| {
            eprintln!("Couldn't read message : {err}");
            let _ = message.send(Message::ClientDisConnected{author_add});
        })?;
        message.send(Message::NewMessage{author_add, bytes:buffer[0..n].to_vec()}).map_err(|err| {
            eprintln!("couldn't send message to server thread {err}")
        })?;
    };

}

fn server(messages: Receiver<Message>)->Result<()>{
    let mut clients: HashMap<SocketAddr,Client> = HashMap::new();
    let mut banned_mfs: HashMap<IpAddr,SystemTime> = HashMap::new();

    loop{
        let msg = messages.recv().expect("Server Recived Message");
        match msg{
            Message::ClientConnected{author} => {
                let auth_add = author.peer_addr().expect("Peer Address Pending");
                let now = SystemTime::now();

                match banned_mfs.get(&auth_add.ip()){
                    Some(banned_at) =>{
                        let duration = now.duration_since(*banned_at).expect(" Dont Crash ");
                        if duration >= BAN_LIMIT{
                            banned_mfs.remove(&auth_add.ip());
                        }else{
                            writeln!(author.as_ref(),"You are Banned!");
                            let _ = author.shutdown(Shutdown::Both);
                        }
                    },
                    None =>{
                        clients.insert(auth_add.clone(),Client{
                            conn: author.clone(),
                            last_message: now,
                            strike_count: 0
                        });
                    }
                };

            },
            Message::ClientDisConnected{author_add} => {
                //let add = author.peer_addr().expect("Peer Address Pending");
                clients.remove(&author_add);
            },
            Message::NewMessage{author_add,bytes} => {
                //let author_add = author.peer_addr().expect("Peer Address Pending");

                if let Some(author) = clients.get_mut(&author_add){
                    let now = SystemTime::now();
                    let diff = now.duration_since(author.last_message).expect(" Dont Crash ");

                    if diff >= MESSAGE_RATE && str::from_utf8(&bytes).is_ok() {
                        for (add,client) in clients.iter(){
                            if *add != author_add{
                                let _ = client.conn.as_ref().write(&bytes);
                            }
                        }
                    } else{
                        author.strike_count+=1;
                        if author.strike_count > STRIKE_COUNT{
                            banned_mfs.insert(author_add.ip().clone(),now);
                            writeln!(author.conn.as_ref(),"You are Banned!");
                            let _ = author.conn.shutdown(Shutdown::Both);
                        }
                    }

                }
            }
        }
    }
}

fn main() -> Result<()>{
    let listener = TcpListener::bind(ADDRESS).map_err(|err| {
        eprintln!("couldn't bind {ADDRESS} : {:?}",err);
    })?;
    println!("INFO: listening to {}", Sensitive::new(ADDRESS));
    let (message_sender, message_reciver) = channel();
    thread::spawn(|| server(message_reciver));

    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                let message_sender = message_sender.clone();
                thread::spawn(|| client(Arc::new(stream),message_sender));
            },
            Err(err) => eprint!("Can't Accept Conection {:?}", err)
        };
    }
    Ok(())
}
