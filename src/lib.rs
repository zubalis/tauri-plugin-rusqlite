use tauri::{
  command, plugin::{Builder, TauriPlugin}, Manager, Runtime, State
};

use crate::types::{Result, ResultList};
use commands::{
    batch::execute_batch, migration::execute_migration, select::execute_select,
    update::execute_update,
};
use error::Error;
use rusqlite::{Connection, OpenFlags};
use serde_json::Value as JsonValue;
use std::{collections::HashMap, sync::Mutex};
use types::Migrations;

pub mod commands;
mod common;
mod error;
mod types;

#[derive(Default)]
pub struct ConfigState(pub Mutex<HashMap<String, Connection>>);

#[command]
async fn open_in_memory(state: State<'_, ConfigState>, name: String) -> Result<()> {
    let connection = Connection::open_in_memory()
        .map_err(|error| Error::OpeningConnection(error.to_string()))?;

    insert_connection(state, connection, name)
}

#[command]
async fn open_in_path(state: State<'_, ConfigState>, path: String) -> Result<()> {
    let connection = Connection::open_with_flags(path.clone(), OpenFlags::default())
        .map_err(|error| Error::OpeningConnection(error.to_string()))?;

    insert_connection(state, connection, path)
}

fn insert_connection(
    state: State<'_, ConfigState>,
    connection: Connection,
    name: String,
) -> Result<()> {
    let mut connections = state.0.lock().unwrap();
    let contains_key = connections.contains_key(&name);

    if !contains_key {
        connections.insert(name.clone(), connection);
    }

    Ok(())
}

#[command]
async fn migration(
    state: State<'_, ConfigState>,
    name: String,
    migrations: Migrations,
) -> Result<()> {
    let connections = state.0.lock().unwrap();
    let connection = match connections.get(&name) {
        Some(connection) => connection,
        None => return Err(Error::Connection()),
    };

    execute_migration(connection, migrations)
}

#[command]
async fn update(
    state: State<'_, ConfigState>,
    name: String,
    sql: String,
    parameters: HashMap<String, JsonValue>,
) -> Result<()> {
    let connections = state.0.lock().unwrap();
    let connection = match connections.get(&name) {
        Some(connection) => connection,
        None => return Err(Error::Connection()),
    };

    execute_update(connection, sql, parameters)
}

#[command]
async fn select(
    state: State<'_, ConfigState>,
    name: String,
    sql: String,
    parameters: HashMap<String, JsonValue>,
) -> Result<ResultList> {
    let connections = state.0.lock().unwrap();
    let connection = match connections.get(&name) {
        Some(connection) => connection,
        None => return Err(Error::Connection()),
    };

    execute_select(connection, sql, parameters)
}

#[command]
async fn batch(state: State<'_, ConfigState>, name: String, sql: String) -> Result<()> {
    let connections = state.0.lock().unwrap();
    let connection = match connections.get(&name) {
        Some(connection) => connection,
        None => return Err(Error::Connection()),
    };

    execute_batch(connection, sql)
}

#[command]
async fn close(state: State<'_, ConfigState>, name: String) -> Result<()> {
    let mut connections = state.0.lock().unwrap();
    let connection = match connections.remove(&name) {
        Some(connection) => connection,
        None => return Err(Error::Connection()),
    };

    connection
        .close()
        .map_err(|(_, error)| Error::ClosingConnection(error.to_string()))?;
    Ok(())
}

/// Extensions to [`tauri::App`], [`tauri::AppHandle`], [`tauri::WebviewWindow`], [`tauri::Webview`] and [`tauri::Window`] to access the clipboard APIs.
pub trait RusqliteExt<R: Runtime> {
    fn rusqlite(&self) -> &ConfigState;
}

impl<R: Runtime, T: Manager<R>> crate::RusqliteExt<R> for T {
    fn rusqlite(&self) -> &ConfigState {
        self.state::<ConfigState>().inner()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("rusqlite")
        .invoke_handler(tauri::generate_handler![
            open_in_memory,
            open_in_path,
            migration,
            update,
            select,
            batch,
            close
        ])
        .setup(|app, _api| {
            app.manage(ConfigState::default());
            Ok(())
        })
        .build()
}
