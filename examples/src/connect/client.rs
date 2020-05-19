//! Simple websocket client.
//!
#[macro_use]
extern crate log;
#[macro_use]
extern crate clap;

use std::time::Duration;
use std::{io, thread};
use std::path::Path;

use actix::*;
use actix::io::SinkWrite;
use actix_codec::Framed;
use awc::{error::WsProtocolError, ws::{Codec, Frame, Message, CloseReason, CloseCode}, Client, BoxedSocket};
use bytes::{Bytes, BytesMut};
use futures::stream::{SplitSink, StreamExt};
use serde_json::json;
use clap::{App, Arg};
use router::token::Token;
use awc::http::Method;
use tokio::fs::File;
use tokio::prelude::*;


#[derive(Clone, Debug)]
struct Remote {
    ws_url: String,
    token_url: String,
}

fn main() {
    let mut server = Remote { ws_url: String::from("http://127.0.0.1:7878/ws"), token_url: String::from("http://127.0.0.1:7878/renew") };
    setup_cmd_lines(&mut server);
    println!("remote server: {:?}", server);

    let sys = System::new("game");

    Arbiter::spawn(async move {
        let token = renew(server.token_url.as_str()).await.unwrap();

        let addr = upgrade(&token, server.ws_url.as_str()).await;

        crtlc(addr.clone()).await;

        console(addr.clone()).await;
    });

    sys.run().unwrap();

    println!("quit now")
}

fn setup_cmd_lines(server: &mut Remote) {
    let m = App::new("client")
        .version("1.0")
        .author("duoduo <25574522@qq.com>")
        .arg(
            Arg::with_name("host")
                .short("h")
                .long("host")
                .value_name("HOST")
                .help("remote ip:port")
                .takes_value(true)
                .default_value("127.0.0.1:7878")
        )
        .get_matches();
    let host = m.value_of("host").unwrap();
    server.ws_url = format!("http://{}/ws", host);
    server.token_url = format!("http://{}/renew", host);
}

struct ChatClient(
    SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>,
);

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand(String);

impl Actor for ChatClient {
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {

        // start heartbeats otherwise server will disconnect after 10 seconds
        self.hb(ctx);
    }

    fn stopping(&mut self, _ctx: &mut Self::Context) -> Running {
        debug!("stopping...");
        Running::Stop
    }

    fn stopped(&mut self, _: &mut Context<Self>) {
        info!("Disconnected");

        // Stop application on disconnect
        System::current().stop();
    }
}

//-------------------------
#[derive(Message)]
#[rtype(result = "()")]
struct Kill;

impl Handler<Kill> for ChatClient {
    type Result = ();

    fn handle(&mut self, _msg: Kill, _: &mut Context<Self>) -> Self::Result {
        self.0.write(Message::Close(Option::Some(CloseReason { code: CloseCode::Normal, description: None }))).unwrap();
    }
}

impl ChatClient {
    fn hb(&self, ctx: &mut Context<Self>) {
        ctx.run_later(Duration::new(10, 0), |act, ctx| {
            act.0.write(Message::Ping(Bytes::from_static(b""))).unwrap();
            act.hb(ctx);

            // client should also check for a timeout here, similar to the
            // server code
        });
    }
}

impl Handler<ClientCommand> for ChatClient {
    type Result = ();

    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        self.0.write(Message::Text(msg.0)).unwrap();
    }
}

impl StreamHandler<Result<Frame, WsProtocolError>> for ChatClient {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        if let Ok(Frame::Text(txt)) = msg {
            debug!("Server: {:?}", txt)
        }
    }

    fn started(&mut self, _ctx: &mut Context<Self>) {
        info!("Connected");
    }

    fn finished(&mut self, ctx: &mut Context<Self>) {
        info!("Server disconnected");
        ctx.stop()
    }
}

impl actix::io::WriteHandler<WsProtocolError> for ChatClient {}

async fn upgrade(token: &Token, url: &str) -> Addr<ChatClient> {
    let (response, framed) = Client::new()
        .ws(url)
        .set_header("Authorization", String::from(&token.token))
        .connect()
        .await
        .expect("websocket connection failed");
    debug!("{:#?}", response);

    let (sink, stream) = framed.split();
    ChatClient::create(|ctx| {
        ChatClient::add_stream(stream, ctx);
        ChatClient(SinkWrite::new(sink, ctx))
    })
}

async fn console(addr: Addr<ChatClient>) {
    thread::spawn(move || loop {
        let mut cmd = String::new();
        if io::stdin().read_line(&mut cmd).is_err() {
            error!("error");
            return;
        }
        addr.do_send(ClientCommand(cmd));
    });
}

async fn crtlc(addr: Addr<ChatClient>) {
    ctrlc::set_handler(move || {
        addr.do_send(Kill);
    }).expect("Error setting Ctrl-C handler");
}

async fn renew(url: &str) -> io::Result<Token> {

    let secret = std::env::current_dir().unwrap().join(Path::new("credential.txt"));
    if let Ok(mut f) = File::open(&secret).await {
        let mut buf = String::new();
        f.read_to_string(&mut buf).await?;

        // let mut buf = String::new();
        // BufReader::new(f).read_line(&mut buf).await;
        if buf.len() > 0 {
            debug!("read token from file: {}", &buf);
            return Ok(Token { token: buf });
        }
    }
    let mut file = File::create(&secret).await?;
    let payload = json!({"phone": "+8618500863838", "password": "123456"});
    let mut res = Client::new()
        .post(url)
        .send_json(&payload)
        .await
        .expect("token renew failed");

    let mut body = BytesMut::new();
    while let Some(chunk) = res.next().await {
        body.extend_from_slice(&chunk.unwrap());
    }

    let token = serde_json::from_slice::<Token>(&body).expect("renew token in bad schema");

    file.write_all(token.token.as_bytes()).await?;
    info!("request a new token: {:?}", token);
    Ok(token)
}