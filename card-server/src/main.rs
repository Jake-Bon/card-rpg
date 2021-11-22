use std::io::{Write, Read, Result};
use std::thread;
use std::time::Duration;
use std::net::{TcpListener, TcpStream, Shutdown};

fn main() -> Result<()>{
    
    
    loop {
    
        let (mut player_1_stream, mut player_2_stream) = accept_clients("127.0.0.1:7878").unwrap(); // localhost
        //let (mut player_1_stream, mut player_2_stream) = accept_clients("34.227.148.203:54345").unwrap();
        
        player_1_stream.set_nonblocking(true);
        player_2_stream.set_nonblocking(true);
        
        let mut p1_buffer = [0; 1024];
        let mut p2_buffer = [0; 1024];
        
        player_1_stream.write_all(b"connected to server as player 1");
        player_1_stream.flush();
        player_2_stream.write_all(b"connected to server as player 2");
        player_2_stream.flush();
        //let mut player_1_stream = player_1_stream.unwrap();
        //let mut player_2_stream = player_2_stream.unwrap();
        //accept("127.0.0.1:7878")?;
    
    
        
        loop {
        
            //if String::from_utf8_lossy(&p1_buffer) != "Quit" && String::from_utf8_lossy(&p2_buffer) != "Quit" {
                
                //println!("String::from_utf8_lossy(&p1_buffer) != 'Quit': {}", String::from_utf8_lossy(&p1_buffer) != "Quit");
                //println!("'{}' VS '{}'", String::from_utf8_lossy(&p1_buffer), "Quit");
                
                //if player_1_stream.peek(&mut p1_buffer).unwrap() > 0 {
                    //player_1_stream.flush()?;
                match player_1_stream.read(&mut p1_buffer) {
                    Ok(T) => {
                        if T > 0 {
                            println!("Received '{}' from player 1", String::from_utf8_lossy(&p1_buffer));
                            player_2_stream.write_all(&p1_buffer);
                            player_2_stream.flush()?;
                            // clear the buffer
                            p1_buffer = [0;1024];
                        }
                        else{
                            println!("Is p1 connection gone?");
                            println!("A client broke their connection to the server!");
                            println!("closing other connections...");
                    
                            player_1_stream.write_all(b"closing connection...");
                            player_1_stream.flush();
                            player_1_stream.shutdown(Shutdown::Both);
                            player_2_stream.write_all(b"closing connection...");
                            player_2_stream.flush();
                            player_2_stream.shutdown(Shutdown::Both);
                            break;
                        }
                    }
                    Err(ref e) => {},
                    Err(e) => { println!("received error: {}", e); },
                    
                    }
                //}
                //}
                
                //if player_2_stream.peek(&mut p2_buffer).unwrap() > 0 {
                    //player_2_stream.flush()?;
                match player_2_stream.read(&mut p2_buffer){
                    Ok(T) => {
                        if T > 0 {
                            println!("Received '{}' from player 2", String::from_utf8_lossy(&p2_buffer));
                            println!("String::from_utf8_lossy(&p1_buffer) != 'Quit': {}", String::from_utf8_lossy(&p2_buffer) != "Quit");
                            println!("'{}' VS '{}'", String::from_utf8_lossy(&p2_buffer), "Quit");
                            player_1_stream.write_all(&p2_buffer);
                            player_1_stream.flush()?;
                            // clear the buffer
                            p2_buffer = [0;1024];
                        }
                        else{
                            println!("Is p2 connection gone?");
                            println!("A client broke their connection to the server!");
                            println!("closing other connections...");
                    
                            player_1_stream.write_all(b"closing connection...");
                            player_1_stream.flush();
                            player_1_stream.shutdown(Shutdown::Both);
                            player_2_stream.write_all(b"closing connection...");
                            player_2_stream.flush();
                            player_2_stream.shutdown(Shutdown::Both);
                            break;
                        }
                    }
                    Err(ref e) => {},
                    Err(e) => { println!("received error: {}", e); },
                }
                //}
                
            //}
            //else {
            //}
            
        }
    
    }
    
    Ok(())
}

fn handle_connection(stream: &mut TcpStream) -> Result<()>{
    //thread::sleep(Duration::from_secs(4));
    stream.write(b"Hi")?;
    Ok(())
}

fn accept_clients(server_addr: &str) -> Result<(TcpStream, TcpStream)> {
    let listener = TcpListener::bind(server_addr)?;
    
    let mut connection_1 : Option<TcpStream> = None;
    let mut connection_1_set = false;
    let mut connection_2 : Option<TcpStream> = None;
    let mut connection_2_set = false;
    
    println!("beginning to accept clients...");
    
    for stream in listener.incoming() {
        
        let stream = stream.unwrap();
        
        // currently will probably hang if one gets accepted as connection 1 and then closes the program
        
        if !connection_1_set {
            connection_1 = Some(stream.try_clone().unwrap());
            //connection_1.unwrap().write(b"connected as player 1");
            println!("Address '{}' is connected as player 1", connection_1.as_ref().unwrap().peer_addr()?);
            connection_1_set = true;
        }
        else if !connection_2_set {
            connection_2 = Some(stream.try_clone().unwrap());
            //connection_2.unwrap().write(b"connected as player 2");
            println!("Address '{}' is connected as player 2", connection_2.as_ref().unwrap().peer_addr()?);
            connection_2_set = true;
        }
        if connection_1_set && connection_2_set {
            println!("both clients connected!");
            // stop accepting new clients
            break;
        }
        
    }
    
    Ok((connection_1.unwrap(), connection_2.unwrap()))
    
}

/*
fn accept(address: &str) -> Result<()> {
    let listener = TcpListener::bind(address)?;
    for stream in listener.incoming() {
        let mut stream = stream?;
        handle_connection(&mut stream)?;
        println!("Connection from: {}", stream.peer_addr()?);
    }
    Ok(())
}
*/
