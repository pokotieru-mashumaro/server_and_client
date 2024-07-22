use std::{io::{stdin, BufRead, BufReader, Write}, net::TcpStream, thread, time::Duration};

fn main() {
    let serv_addr = "127.0.0.1:8888";
    let mut socket = TcpStream::connect(serv_addr).expect("サーバーと接続できません");
    socket.set_nonblocking(true).expect("利用不可");
    println!("{}と接続しました", serv_addr);
    start_thread(socket.try_clone().unwrap());

    let user = input("お名前は?");
    println!("{}さん、メッセージを入力してください", user);
    loop {
         let msg = input("");
         let msg = format!("{}> {}\n", user, msg);
         let buf = msg.as_bytes();
         socket.write_all(buf).unwrap();
    }
}

fn start_thread(socket: TcpStream) {
    let mut reader = BufReader::new(socket);
    thread::spawn(move || loop {
        let mut buf = String::new();
        if let Ok(n) = reader.read_line(&mut buf) {
            if n > 0 {
                println!("[受信] {}" ,buf.trim());
            }
        }
        thread::sleep(Duration::from_millis(100));
    });
}

fn input(msg: &str) -> String {
    if msg != "" { 
        println!("{}", msg);
    }
    let mut buf = String::new();
    stdin().read_line(&mut buf).expect("標準入力エラー");
    String::from(buf.trim())
}