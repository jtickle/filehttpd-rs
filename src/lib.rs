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

use std::env;
use tokio::io::{BufReader, AsyncBufReadExt, BufWriter, AsyncWriteExt};
use tokio::net::TcpStream;
use tracing::{warn, info, trace};
use anyhow::Result;

pub mod request;

#[derive(Debug, Clone)]
pub enum HttpMethod {
    GET,
    HEAD,
//    POST,
//    PUT,
//    DELETE,
//    CONNECT,
//    OPTIONS,
//    TRACE
}

#[derive(Debug, Clone)]
pub enum HttpVersion {
    OneOh,
    OneOne,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub web_root: String,
    pub base_uri: String,
    pub directory_index: Vec<String>,
}

impl Config {
    pub fn build() -> Config {
        let web_root = env::var("FILEHTTPD_WEB_ROOT")
            .unwrap_or(".".to_string());

        let base_uri = env::var("FILEHTTPD_BASE_URI")
            .unwrap_or("/".to_string());

        let directory_index_str = env::var("FILEHTTPD_DIRECTORY_INDEX")
            .unwrap_or("index.htm index.html".to_string());

        let directory_index:Vec<String> = directory_index_str.split_whitespace().map(str::to_string).collect();

        let c = Config { web_root, base_uri, directory_index };
        info!("Configuration Loaded: {:#?}", c);
        c
    }
}

pub async fn handle_client(config: &Config, mut socket: TcpStream, _con_no: u64) -> Result<()> {
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
            info!("Reading index.html...");

            match tokio::fs::read("root/index.html").await {
                Err(e) => {
                    warn!("Error occurred reading file: {:#?}", e);
                    writer.write(b"HTTP/1.1 418 I'm a teapot\r\n").await?;
                    writer.write(b"Content-Type: text/plain\r\n").await?;
                    writer.write(b"\r\n").await?;
                    writer.write(b"Stubbornly refusing to brew coffee with a teapot.\r\n").await?;
                }
                Ok(data) => {
                    info!("Sending contents of index.html length {}...", data.len());
                    writer.write(b"HTTP/1.1 200 OK\r\n").await?;
                    writer.write(b"Content-Type: text/html\r\n").await?;
                    writer.write(format!("Content-Length: {}\r\n", data.len()).as_bytes()).await?;
                    writer.write(b"\r\n").await?;
                    writer.write(&data).await?;
                }
            };

            writer.flush().await?;
            break;
        }
    }

    Ok(())
}