use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;
use actix_web_actors::ws::{CloseCode, CloseReason};
use awc::{
    error::WsProtocolError,
    ws::{Codec, Frame, Message},
    BoxedSocket, Client,
};
use futures::stream::{SplitSink, StreamExt};
use std::{io, thread};

struct WebsocketClientActor(SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>);
impl Actor for WebsocketClientActor {
    type Context = Context<Self>;
}
// User Input Handling
fn message_from_str(cmd: String) -> Message {
    if cmd == "exit" {
        Message::Close(Some(CloseReason::from(CloseCode::Normal)))
    } else {
        Message::Text(cmd)
    }
}
impl Handler<ClientCommand> for WebsocketClientActor {
    type Result = ();
    fn handle(&mut self, msg: ClientCommand, _ctx: &mut Context<Self>) {
        self.0.write(message_from_str(msg.0));
    }
}
// Server Response Handling
impl StreamHandler<Result<Frame, WsProtocolError>> for WebsocketClientActor {
    fn handle(&mut self, msg: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
        if let Ok(Frame::Text(txt)) = msg {
            println!("From Server: {:?}", txt)
        }
    }
    fn started(&mut self, _ctx: &mut Context<Self>) {
        println!("client connected with server");
    }
    fn finished(&mut self, ctx: &mut Context<Self>) {
        println!("connection finished");
        ctx.stop()
    }
}
impl actix::io::WriteHandler<WsProtocolError> for WebsocketClientActor {}

#[derive(Message, Debug)]
#[rtype(result = "()")]
struct ClientCommand(String);

fn create_ws_client_actor(framed: Framed<BoxedSocket, Codec>) -> Addr<WebsocketClientActor> {
    let (sink, stream) = framed.split();
    WebsocketClientActor::create(|ctx| {
        WebsocketClientActor::add_stream(stream, ctx);
        WebsocketClientActor(SinkWrite::new(sink, ctx))
    })
}

fn main() -> eyre::Result<()> {
    let sys = System::new("websocket-client");
    Arbiter::spawn(async {
        let ws_connect = Client::new()
            .ws("http://localhost:8080/ws/")
            .connect()
            .await;
        if let Ok((_response, framed)) = ws_connect {
            let addr = create_ws_client_actor(framed);
            thread::spawn(move || loop {
                let mut user_input = String::new();
                if io::stdin().read_line(&mut user_input).is_err() {
                    println!("input error, please re-input");
                    continue;
                }
                addr.do_send(ClientCommand(user_input.trim().to_string()));
            });
        }
    });
    sys.run()?;
    Ok(())
}
