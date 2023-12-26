// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use lazy_static::lazy_static;
use mysql::{Pool, PooledConn};
use mysql::prelude::*;
use mysql::params;
use serde::{Deserialize, Serialize};

lazy_static! {
    static ref DB_POOL: Pool = connect_db();
}

#[derive(Debug, Deserialize, Serialize)]
struct Note {
    id: Option<u32>,
    title: String,
    description: String,
    created_at: String,
    updated_at: String,
}

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn custom_command() -> String {
    format!("This is invoked from vue application")
}

#[tauri::command]
fn save_note(note: Note) -> String {

    let new_note = Note {
        id: None,
        title : note.title.to_string(),
        description : note.description.to_string(),
        created_at : note.created_at.to_string(),
        updated_at : note.updated_at.to_string()
    };

    insert_note(&new_note);

    format!("saviing a note")
}


#[tauri::command]
fn get_all_notes() -> Vec<Note> {
    get_notes()
}

fn connect_db() ->mysql::Pool {
    let url = "mysql://root@localhost:3306/notes";
    let pool = Pool::new(url).unwrap();
    pool
}

fn insert_note(note: &Note) {
    let mut conn = DB_POOL.get_conn().expect("Failed to get database connection");

    let inserted_id: Option<i32> = conn.exec_first(
        "INSERT INTO notes (title, description, created_at, updated_at) VALUES (:title, :description, :created_at, :updated_at)",
        params! {
            "title" => &note.title,
            "description" => &note.description,
            "created_at" => &note.created_at,
            "updated_at" => &note.updated_at,
        },
    )
    .expect("Failed to execute INSERT query");
}

fn get_notes() -> Vec<Note> {
    let mut conn = DB_POOL.get_conn().expect("Failed to get database connection");

    let notes: Vec<Note> = conn.query_map("SELECT * FROM notes",
                                          |(id, title, description, created_at, updated_at)| Note {
                                              id,
                                              title,
                                              description,
                                              created_at,
                                              updated_at,
                                          },)     
        .expect("Failed to execute INSERT query");

        notes
        // serde_json::to_string(&notes).expect("Failed to Serialize notes")
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, custom_command, save_note, get_all_notes ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
