use async_std::io::{BufReader, Lines, ReadExt, WriteExt};
use async_std::io::prelude::BufReadExt;
use async_std::net::{TcpListener, TcpStream};
use async_std::stream::StreamExt;
use async_std::task;

const SERVER_ADDR: &str = "localhost:8888";

/// Async SERVER LOGGER or HTTP REDIRECT UTILITY or whatever you want it to be (just write the handler)
/// One thread, blazingly fast!
/// Connect from the browser (type localhost:8888 or SERVER_ADDR) to see some magic

fn main() {
    task::block_on(async {
        let s = match TcpListener::bind(SERVER_ADDR).await {
            Ok(s) => s,
            Err(e) => panic!("No reason to live if a server cannot bind: {}", e),
        };
        println!("Listening on {}!", SERVER_ADDR);
        println!("Type {} on your browser!!!!!", SERVER_ADDR);

        let mut inc = s.incoming();
        // does not block
        while let Some(cli) = inc.next().await {
            match cli {
                Ok(cli) => {
                    println!("New client {:?}!", cli.peer_addr());
                    // spawn a task in the current thread (does not block)
                    // task::spawn(print_client_stuff(cli));
                    task::spawn(redirect_http(cli));
                }
                Err(e) => { eprintln!("{}", e); break; }
            }
        }
    })
}

fn lines(client_stream: TcpStream) -> Lines<BufReader<TcpStream>> {
    let reader = BufReader::new(client_stream);
    reader.lines()
}

/// Streamlined reading
/// Nothing here blocks
async fn print_client_stuff(client_stream: TcpStream) {
    let mut lines = lines(client_stream);
    while let Some(line)= lines.next().await {
        match line {
            Ok(line) => println!("{}", line),
            Err(e) => { eprintln!("{}", e); break; }
        }
    }
}

async fn redirect_http(mut client_stream: TcpStream) {
    // to avoid cache, temporary redirect
    let victim_payload =  "HTTP/1.1 307 Temporary Redirect\r\nLocation: https://www.shorturl.at/aijz8\r\n\r\n";
    client_stream.write(victim_payload.as_bytes()).await.unwrap();
    client_stream.flush().await.unwrap();
}
