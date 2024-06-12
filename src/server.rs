use crate::model::{
  stickdeck_server::{Stickdeck, StickdeckServer},
  Gamepad, UpdateGamepadReply,
};
use std::net::SocketAddr;
use tokio::sync::mpsc;
use tonic::{transport::Server, Request, Response, Status};

pub enum Action {
  UpdateGamepad(Gamepad),
}

#[derive(Debug)]
pub struct TheStickdeckServer {
  action_tx: mpsc::Sender<Action>,
}

#[tonic::async_trait]
impl Stickdeck for TheStickdeckServer {
  async fn update_gamepad(
    &self,
    request: Request<Gamepad>,
  ) -> Result<Response<UpdateGamepadReply>, Status> {
    let gamepad = request.into_inner();

    self
      .action_tx
      .send(Action::UpdateGamepad(gamepad))
      .await
      .expect("Failed to send action to the main loop");

    Ok(Response::new(UpdateGamepadReply {}))
  }
}

impl TheStickdeckServer {
  pub async fn start(port: u16, action_tx: mpsc::Sender<Action>) {
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    let stickdeck = Self { action_tx };

    Server::builder()
      .add_service(StickdeckServer::new(stickdeck))
      .serve(addr)
      .await
      .expect("Failed to start the server");
  }
}
