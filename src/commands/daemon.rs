use std::path::Path;

use interprocess::local_socket::tokio::{LocalSocketListener, LocalSocketStream};

use crate::{constants::APP_NAME, error::Result};

struct IpcServer {
    listener: LocalSocketListener,
}

impl IpcServer {
    fn new(listener: LocalSocketListener) -> Self {
        Self { listener }
    }

    fn create() -> Result<Self> {
        if cfg!(unix) {
            let path = format!("/run/{}/{}", APP_NAME, APP_NAME);
            let path = Path::new(&path).with_extension("sock");

            let listener = LocalSocketListener::bind(path)?;

            Ok(Self::new(listener))
        } else {
            unimplemented!()
        }
    }

    async fn accept(&self) -> Result<LocalSocketStream> {
        let client = self.listener.accept().await?;

        Ok(client)
    }

    // async fn handle_client() -> Result<()> {}
}

pub async fn start_daemon() -> Result<()> {
    let server = IpcServer::create()?;

    while let Ok(client_stream) = server.accept().await {
        println!("{}", client_stream.peer_pid().unwrap())
    }

    Ok(())
}
