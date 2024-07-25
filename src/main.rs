mod ch5;
use ch5::client::client_main;
use ch5::server::server_main;

use std::env;

fn main() {
    let av: Vec<String> = env::args().collect();

    match av.len() {
        2 => {
            match &*av[1] {
                "server" => server_main(),
                "client" => client_main(),
                _ => {
                    println!("Bad Args😡😡😡");
                    println!("Args: server client");
                }
            }
        }
        _ => {
            println!("Give two Args😡😡😡");
            println!("Args: server client");
        }
    }

}
