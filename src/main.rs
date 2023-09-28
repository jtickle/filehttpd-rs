// Copyright (C) 2023 Jeffrey W. Tickle
// This file is part of filehttpd-rs <https://github.com/jtickle/filehttpd-rs>.
//
// filehttpd-rs is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// filehttpd-rs is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with filehttpd-rs. If not, see <http://www.gnu.org/licenses/>.

use std::io::{BufReader, BufRead, BufWriter, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(stream: TcpStream) -> std::io::Result<()> {
    println!("Connection established to peer {}", stream.peer_addr()?);

    let mut reader = BufReader::new(stream.try_clone()?);
    let mut writer = BufWriter::new(stream.try_clone()?);

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;

        if len == 0 {
            println!("Connection closed.");
            break;
        }

        println!("STREAM ({}): {:#?}", len, line);

        if line == "\r\n" {
            println!("Sending response...");
            writer.write(b"HTTP/1.1 200 OK\r\n")?;
            writer.write(b"Content-Type: text/html\r\n")?;
            writer.write(b"\r\n")?;
            writer.write(b"Hello there!\r\n")?;
            writer.flush()?;
            break;
        }
    }

    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?)?;
    }

    Ok(())
}
