use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock, Weak};

use russh::client::Msg;
use russh::Channel;
use serde::Serialize;
use uuid::Uuid;
use vt100::Parser;

use crate::session_manager::connection::ConnectionsMap;
use connection::Connection;

mod connection;
mod device;
mod error;
mod handler;
mod manager;
mod shell;

#[derive(Default)]
pub struct SessionManager {
    pub(crate) shells: RwLock<HashMap<ShellToken, Arc<Shell>>>,
    connections: Arc<Mutex<ConnectionsMap>>,
}

pub struct Shell {
    pub token: ShellToken,
    connection: Weak<Connection>,
    pub(crate) channel: Mutex<Channel<Msg>>,
    pub(crate) parser: Mutex<Parser>,
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct ShellToken {
    pub connection_id: Uuid,
    pub channel_id: String,
}

#[derive(Clone, Serialize, Debug)]
pub struct ShellBuffer {
    rows: Vec<Vec<u8>>,
    cursor: (u16, u16),
}

#[derive(Debug, Serialize, Clone)]
pub struct Error {
    message: String,
}
