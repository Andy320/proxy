use std::ops::Deref;
use std::time::Instant;
use actix::prelude::*;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix_web::error::{ErrorBadRequest, ErrorUnauthorized};
use validator::Validate;
use futures::executor::block_on;
use crate::token::{do_decode, Token, do_encode};
use crate::grpc::client::GrpcClient;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct IDObject {
    #[validate(phone)]
    pub phone: String,
    #[validate(non_control_character)]
    #[validate(length(min = 6, max = 16))]
    password: String,
}

pub async fn start(gc: GrpcClient) -> crate::error::Result<()> {
    // http server -> application -> service -> resource -> handler
    info!("start http server ...");
    HttpServer::new( move || {
        App::new()
            .data(gc.clone())
            .wrap(middleware::Logger::default())
            .service(web::resource("/renew")
                .data(web::JsonConfig::default())
                .route(web::post().to(renew)))
            .service(web::resource("/ws")
                .route(web::get().to(ws_index)))
    })
        .bind(*crate::HTTP_ADDR)?
        .run()
        .await?;

    Ok(())
}

async fn ws_index(r: HttpRequest, stream: web::Payload, data: web::Data<GrpcClient>) -> Result<HttpResponse, Error> {
    debug!("request is: {:?}", r);
    // let s = data.get_ref();
    let value = r.headers()
        .get("Authorization")
        .ok_or(ErrorUnauthorized(""))?
        .to_str()
        .map_err(ErrorBadRequest)?;
    do_decode(value).map_err(ErrorUnauthorized)?;
    debug!("token verification successful");
    ws::start(MyWebSocket::new(data.get_ref()), &r, stream)
}

async fn renew(item: web::Json<IDObject>, req: HttpRequest) -> Result<HttpResponse, Error> {
    debug!("client request: {:?}", req);
    debug!("json body: {:#?}", &item);
    let o = item.deref();
    o.validate().map_err(ErrorBadRequest)?;
    let token = do_encode(o.phone.as_str()).map_err(ErrorBadRequest)?;
    debug!("token: {}", token);
    Ok(HttpResponse::Ok().json(Token { token }))
}

struct MyWebSocket {
    hb_now: Instant,
    gc: GrpcClient,
}

impl MyWebSocket {
    fn new(s: &GrpcClient) -> Self {
        Self { hb_now: Instant::now(), gc: s.clone() }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(*crate::WS_HEARTBEAT, move |act, ctx| {
            if Instant::now().duration_since(act.hb_now) > *crate::WS_TIMEOUT {
                error!("Websocket Client heartbeat failed, disconnecting!");
                // stop actor
                ctx.stop();
                return;
            }
//            ctx.ping(b"");
        });
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb_now = Instant::now();
                ctx.pong(&msg);
                let r =  block_on(self.gc.heart_beaten());
                match r {
                    Ok(()) => {info!("call grpc done")},
                    Err(e) => error!("error in grpc call: {:?}", e),
                }
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb_now = Instant::now();
            }
            Ok(ws::Message::Text(mut text)) => {
                text.push_str(" is ok");
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => ctx.binary(bin),
            Ok(ws::Message::Close(_)) => {
                info!("good bye");
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}