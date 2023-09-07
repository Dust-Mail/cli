use clap::Subcommand;

use crate::error::Result;

use self::{daemon::start_daemon, login::handle_login};

mod daemon;
mod login;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Daemon,
    Login,
}

impl Commands {
    pub async fn handle(&self) -> Result<()> {
        match self {
            Self::Daemon => start_daemon().await,
            Self::Login => handle_login().await,
        }
    }
}
