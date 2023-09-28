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

use tokio::io::{BufReader, AsyncBufReadExt, BufWriter, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use anyhow::Result;
use tracing::{error, info, trace};

async fn handle_client(mut socket: TcpStream, _con_no: u64) -> Result<()> {
    let (read_half, write_half) = socket.split();
    let mut reader = BufReader::new(read_half);
    let mut writer = BufWriter::new(write_half);

    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line).await?;

        if len == 0 {
            info!("Connection closed");
            break;
        }

        trace!("STREAM ({}): {:#?}", len, line);

        if line == "\r\n" {
            info!("Sending response...");
            writer.write(b"HTTP/1.1 200 OK\r\n").await?;
            writer.write(b"Content-Type: text/html\r\n").await?;
            writer.write(b"\r\n").await?;
            writer.write(b"Hello there!\r\n").await?;
            writer.flush().await?;
            break;
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    // Set up Tracing
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .finish();
    tracing::subscriber::set_global_default(subscriber)?;

    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    let mut con_count: u64 = 0;

    loop {
        let (socket, addr) = listener.accept().await?;
        let con_no = con_count;
        con_count += 1;
        tokio::spawn(async move {
            info!("Connection received from peer {}", addr);
            match handle_client(socket, con_no).await {
                Ok(()) => {}
                Err(err) => {
                    error!("Error processing request from {}: {}", addr, err);
                }
            }
        });
    }
}
