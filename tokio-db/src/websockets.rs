use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse, web};
use actix_web_actors::ws;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

pub struct AppState {
    pub clients: Arc<Mutex<HashMap<Uuid, Addr<WebSocketSession>>>>,
}

impl AppState {
    #[allow(dead_code)]
    pub fn new() -> Self {
        AppState {
            clients: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub fn send_ping_to_all_clients(&self) {
        let clients = self.clients.lock().unwrap();
        for client in clients.values() {
            client.do_send(SendMessage("Ping".to_string()));
        }
    }
}

pub struct WebSocketSession {
    id: Uuid,
    state: Arc<Mutex<HashMap<Uuid, Addr<WebSocketSession>>>>,
}

impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();
        let mut clients = self.state.lock().unwrap();
        clients.insert(self.id, addr);
        println!(
            "Client {} connected, number of connected players {}",
            self.id,
            clients.len()
        );
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        let mut clients = self.state.lock().unwrap();
        clients.remove(&self.id);
        println!(
            "Client {} disconnected, number of connected players {}",
            self.id,
            clients.len()
        );
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                println!("Received: {}", text);
            }
            Ok(ws::Message::Close(_reason)) => {
                ctx.stop();
            }
            _ => (),
        }
    }
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendMessage(pub String);

impl Handler<SendMessage> for WebSocketSession {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

pub async fn ws_index(
    r: HttpRequest,
    stream: web::Payload,
    data: web::Data<AppState>,
) -> Result<HttpResponse, Error> {
    let id = Uuid::new_v4();
    let session = WebSocketSession {
        id,
        state: data.clients.clone(),
    };
    ws::start(session, &r, stream)
}
