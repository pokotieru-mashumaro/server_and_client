use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn server_main() {
    let serv_addr = "127.0.0.1:7878";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    let server = TcpListener::bind(serv_addr).expect("サーバー起動失敗");
    server.set_nonblocking(true).expect("利用不可");
    println!("{}でサーバーが起動しました", serv_addr);

    loop {
        if let Ok((client, addr)) = server.accept() {
            println!("クライアントが接続: {}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        if let Ok(msg) = rx.try_recv() {
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            if n > 0 {
                tx.send(msg).unwrap();
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];

    for mut socket in clients.into_iter() {
        let bytes = String::from(s).into_bytes();

        if let Err(e) = socket.write_all(&bytes) {
            println!("送信エラー: {}", e);
            continue;
        }
        collector.push(socket);
    }
    collector
}
