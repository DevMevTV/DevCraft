use std::io::{Read, Write};

fn handle_legacy_ping(mut stream: std::net::TcpStream) {
    // Legacy ping response packet
    let response = build_legacy_ping_response(
        768,                     // Protocol version for compatibility
        "1.21.3",                  // Minecraft server version
        "§4RUST SERVER BE BRRRRRR",        // MOTD
        10,                      // Current players
        100                      // Max players
    );

    // Send the response to the client
    if let Err(e) = stream.write_all(&response) {
        eprintln!("Failed to send legacy ping response: {}", e);
    } else {
        println!("Legacy ping response sent!");
    }
}

fn build_legacy_ping_response(
    protocol_version: i32,
    minecraft_version: &str,
    motd: &str,
    current_players: i32,
    max_players: i32,
) -> Vec<u8> {
    let mut response = Vec::new();

    // Packet ID: 0xFF
    response.push(0xFF);

    // Construct the response string
    let response_string = format!(
        "§1\0{}\0{}\0{}\0{}\0{}",
        protocol_version, minecraft_version, motd, current_players, max_players
    );

    // Encode response string as UTF-16BE
    let utf16_response: Vec<u8> = response_string
        .encode_utf16()
        .flat_map(|c| c.to_be_bytes())
        .collect();

    // Add the length of the UTF-16 string as a big-endian short
    let length = utf16_response.len() / 2; // Length in characters
    response.extend_from_slice(&(length as u16).to_be_bytes());

    // Add the UTF-16 encoded string
    response.extend_from_slice(&utf16_response);

    response
}

fn handle_packets(buffer: [u8; 1024], stream: std::net::TcpStream) {
    match buffer[0] {
        0xFE => {
            println!("Legacy ping detected");
            handle_legacy_ping(stream);
        }
        n => {
            println!("Unexpected packet: {}", n);
        }
    }
}

fn main() {
    let listener = std::net::TcpListener::bind("127.0.0.1:25565").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0; 1024];
                if let Ok(n) = stream.read(&mut buffer) {
                    if n > 0 {
                        handle_packets(buffer, stream);
                    }
                }
            }
            Err(e) => println!("Connection error: {}", e),
        }
    }
}
