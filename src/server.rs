use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use chrono::Utc;
use std::time::Duration;
struct WebSocketActor {}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;
    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocketActor started");
        self.periodic_send(ctx);
    }
    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocketActor stopped");
    }
}
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        println!("WebSocketActor get message");
        match msg {
            Ok(ws::Message::Text(text)) => ctx.text(format!("{{type:echo, text:{}}}", text)),
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl WebSocketActor {
    fn new() -> Self {
        Self {}
    }
    fn periodic_send(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(Duration::from_secs(5), |_act, ctx| {
            let msg = format!("{{type:periodic, time:{}}}", Utc::now().to_string());
            ctx.text(msg);
        });
    }
}

async fn ws_handler(r: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(WebSocketActor::new(), &r, stream)
}

fn configurate_service(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ws/").route(web::get().to(ws_handler)));
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().configure(configurate_service))
        .bind("localhost:8080")?
        .run()
        .await
}
