mod countries;
mod date;
mod game;
mod leagues;
mod r#match;
mod player;
mod routes;
mod teams;
use crate::routes::ServerRoutes;
use core::SimulatorData;
use database::DatabaseEntity;
use log::info;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct FootballSimulatorServer {
    data: GameAppData,
}

impl FootballSimulatorServer {
    pub fn new(data: GameAppData) -> Self {
        FootballSimulatorServer { data }
    }

    pub async fn run(&self) {
        let addr = SocketAddr::from(([0, 0, 0, 0], 18000));

        info!("listen at: http://localhost:18000");

        let app = ServerRoutes::create().with_state(self.data.clone());

        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .unwrap();
    }
}

pub struct GameAppData {
    pub database: Arc<DatabaseEntity>,
    pub data: Arc<Mutex<Option<SimulatorData>>>,
}

impl Clone for GameAppData {
    fn clone(&self) -> Self {
        GameAppData {
            database: Arc::clone(&self.database),
            data: Arc::clone(&self.data),
        }
    }
}
