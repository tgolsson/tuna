use anyhow::Result;
use nanoserde::{DeJson, SerJson};
use std::{
    net::{TcpListener, TcpStream},
    str::FromStr,
};
use tiny_http::{Header, Response as HttpResponse, Server};
use tuna::Tuneable;
use tungstenite::{accept, WebSocket};

use include_dir::{include_dir, Dir};

static PROJECT_DIR: Dir = include_dir!("html");

fn content_type(url: &str) -> Option<Header> {
    if url.ends_with(".js") {
        return Header::from_str("Content-Type: application/javascript; charset=UTF=8").ok();
    }

    if url.ends_with(".css") {
        return Header::from_str("Content-Type: text/css; charset=UTF=8").ok();
    }

    if url.ends_with(".html") {
        return Header::from_str("Content-Type: text/html; charset=UTF=8").ok();
    }

    None
}

#[derive(DeJson, SerJson, Debug)]
enum TunaMessage {
    ListAll,
    Tuneables(tuna::TunaState),
    Delta((String, String, Tuneable)),
    Ok((String, String)),
}

struct TunaClient {
    websocket: WebSocket<TcpStream>,
}

impl TunaClient {
    fn new(stream: TcpStream) -> Result<Self> {
        let websocket = accept(stream)?;

        Ok(Self { websocket })
    }

    fn poll(&mut self) -> bool {
        let msg = self.websocket.read_message().unwrap();

        if msg.is_text() {
            let contents = msg.into_text().unwrap();
            let message: TunaMessage = match DeJson::deserialize_json(&contents) {
                Ok(v) => v,
                Err(e) => {
                    log::error!("failed deserialization: {}, {}", e, contents);
                    return true;
                }
            };

            match message {
                TunaMessage::ListAll => {
                    let state = tuna::TUNA_STATE.read();
                    let res = TunaMessage::Tuneables((*state).clone());

                    let response = SerJson::serialize_json(&res);
                    self.websocket
                        .write_message(tungstenite::Message::Text(response))
                        .unwrap();
                }

                TunaMessage::Delta((category, name, tuneable)) => {
                    tuneable.apply_to(&category, &name);

                    let response = SerJson::serialize_json(&TunaMessage::Ok((category, name)));
                    self.websocket
                        .write_message(tungstenite::Message::Text(response))
                        .unwrap();
                }
                TunaMessage::Tuneables(_) | TunaMessage::Ok((_, _)) => {
                    panic!("unexpected message kind")
                }
            }
        } else if msg.is_close() {
            return false;
        } else {
            log::error!("received non-string message: {:?}", msg);
        }

        true
    }
}

/// The server to tuna web. Will deal with both serving of HTTP content and the
/// websockets used for management.
pub struct TunaServer {
    server: TcpListener,
    http_server: Server,
}

impl TunaServer {
    /// Create a new Tuna Web server. Will serve HTTP on the specified port, and
    /// websocket traffic on the subsequent port (`port + 1`).
    pub fn new(port: u16) -> anyhow::Result<Self> {
        let http_server = Server::http(("0.0.0.0", port))
            .map_err(|e| anyhow::format_err!("http server error: {}", e))?;

        let server = TcpListener::bind(("127.0.0.1", port + 1))?;

        server.set_nonblocking(true)?;

        Ok(Self {
            server,
            http_server,
        })
    }

    /// Update the HTTP server, draining the request queue before returning.
    pub fn work_http(&mut self) {
        loop {
            match self.http_server.try_recv() {
                Ok(Some(req)) => {
                    log::debug!("request: {:#?}", req);
                    let response = match req.url() {
                        "/" => HttpResponse::from_string(
                            PROJECT_DIR
                                .get_file("index.html")
                                .unwrap()
                                .contents_utf8()
                                .unwrap(),
                        )
                        .with_status_code(200)
                        .with_header(content_type(".html").unwrap()),
                        _ => match PROJECT_DIR.get_file(&req.url()[1..]) {
                            Some(contents) => {
                                HttpResponse::from_string(contents.contents_utf8().unwrap())
                                    .with_status_code(200)
                                    .with_header(content_type(req.url()).unwrap())
                            }
                            _ => HttpResponse::from_string("not found").with_status_code(404),
                        },
                    };

                    let _ = req.respond(response);
                }
                Ok(None) => {
                    break;
                }
                Err(e) => {
                    log::error!("Http Error: {:?}", e);
                    break;
                }
            }
        }
    }

    /// Update the websocket server, draining the connection queue before returning.
    pub fn work_websocket(&mut self) {
        loop {
            match self.server.accept() {
                Ok((stream, addr)) => {
                    log::debug!("New Tuna client from: {:?}", addr);

                    match TunaClient::new(stream) {
                        Ok(mut client) => {
                            std::thread::spawn(move || loop {
                                if !client.poll() {
                                    break;
                                }
                            });
                        }
                        Err(e) => log::error!("failed to accept client: {}", e),
                    }
                }
                Err(e) if e.kind() != std::io::ErrorKind::WouldBlock => {
                    log::error!("Error during accept: {:?}", e);
                    break;
                }
                _ => {
                    break;
                }
            }
        }
    }
    /// Update all connections and servers in one go. Note that the clients will
    /// not be polled here as they are blocking.
    pub fn loop_once(&mut self) {
        self.work_http();
        self.work_websocket();
    }
}
