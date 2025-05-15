use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::{postgres::PgPoolOptions, prelude::FromRow, Pool, Postgres};
use tauri::Manager as _;
use tokio::sync::Mutex;
use tauri_plugin_updater::UpdaterExt;

use dotenv::dotenv;
use std::env;

struct AppState {
    db: Option<Pool<Postgres>>,
}

#[derive(Debug, Serialize, Deserialize, sqlx::Type)]
enum NoteStatus {
    NORMAL,
    URGENT,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
struct Note {
    id: i32,
    description: String,
    status: NoteStatus,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_updater::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            connect_to_db,
            add_note,
            get_notes,
            search_note,
            update_note,
            delete_note,
            get_app_version,
            check_for_updates
        ])
        .setup(|app| {
            println!(".setup()");
            app.manage(Mutex::new(AppState {
                db: Default::default(),
            }));
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn check_for_updates(app_handle: tauri::AppHandle) -> Result<(), String> {
    println!("check_for_updates()");
    
    let updater = match app_handle.updater() {
        Ok(updater) => updater,
        Err(e) => {
            eprintln!("Failed to get updater: {:?}", e);
            return Err(e.to_string());
        }
    };

    match updater.check().await {
        Ok(update) => {
            let mut downloaded = 0;
            if let Some(update) = update {
                println!("Update current version: {}", update.current_version);
                println!("Update available: {}", update.version);
                println!("Update download_url: {}", update.download_url);

                // alternatively we could also call update.download() and update.install() separately
                update.download_and_install(
                    |chunk_length, content_length| {
                    downloaded += chunk_length;
                    println!("downloaded {downloaded} from {content_length:?}");
                    },
                    || {
                    println!("download finished");
                    },
                )
                .await.map_err(|e| e.to_string())?;

                println!("update installed");
                app_handle.restart();

            } else {
                println!("No updates available.");
                return Err("No updates available.".to_string());
            }
        }
        Err(e) => {
            eprintln!("Failed to check for updates: {:?}", e);
        }
    }

    Ok(())
}

#[tauri::command]
async fn connect_to_db(
    state: tauri::State<'_, Mutex<AppState>>,
    connectionstr: &str,
) -> Result<(), String> {
    let database_url = if !connectionstr.is_empty() {
        connectionstr.to_string()
    } else {
        dotenv().ok();
        env::var("VITE_DATABASE_URL").expect("VITE_DATABASE_URL must be set")
    };

    println!("connect_to_db url = {}", database_url);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await;

    match pool {
        Ok(db) => {
            println!("connect_to_db success");
            let mut app_state = state.lock().await;

            sqlx::migrate!("./migrations").run(&db).await.unwrap();

            app_state.db = Some(db);
        }
        Err(error) => {
            println!("initial_connect_postgres() Error {}", error);
            return Err(error.to_string());
        }
    }

    Ok(())
}

#[tauri::command]
async fn add_note(
    state: tauri::State<'_, Mutex<AppState>>,
    description: &str,
    status: NoteStatus,
) -> Result<(), String> {
    println!("add_note() desctiption = {}", description);
    let app_state = state.lock().await;
    let db = &<std::option::Option<Pool<Postgres>> as Clone>::clone(&app_state.db)
        .ok_or("DB not seted")?;

    sqlx::query("INSERT INTO notes (description, status) VALUES ($1, $2)")
        .bind(description)
        .bind(status)
        .execute(db)
        .await
        .map_err(|e| {
            println!("Error saving add_note: {}", e);
            format!("Failed to get notes {}", e)
        })?;

    println!("note added");

    Ok(())
}

#[tauri::command]
async fn get_notes(state: tauri::State<'_, Mutex<AppState>>) -> Result<Vec<Note>, String> {
    println!("get_notes()");

    let app_state = state.lock().await;
    let db = &<std::option::Option<Pool<Postgres>> as Clone>::clone(&app_state.db)
        .ok_or("DB not seted")?;

    let notes: Vec<Note> = sqlx::query_as::<_, Note>("SELECT * FROM notes")
        .fetch(db)
        .try_collect()
        .await
        .map_err(|e| {
            println!("Failed to get notes: {}", e);
            format!("Failed to get notes {}", e)
        })?;

    println!("get_notes() done");
    Ok(notes)
}

#[tauri::command]
async fn search_note(
    state: tauri::State<'_, Mutex<AppState>>,
    query: &str,
) -> Result<Vec<Note>, String> {
    println!("serch_notes() query= {} ", query);

    let app_state = state.lock().await;
    let db = &<std::option::Option<Pool<Postgres>> as Clone>::clone(&app_state.db)
        .ok_or("DB not seted")?;

    let notes: Vec<Note> =
        sqlx::query_as::<_, Note>("SELECT * FROM NOTES WHERE DESCRIPTION LIKE $1")
            .bind(format!("%{}%", query))
            .fetch(db)
            .try_collect()
            .await
            .map_err(|e| {
                println!("Failed to search notes: {}", e);
                format!("Failed to search notes {}", e)
            })?;

    println!("serch_notes() size= {}", notes.len());
    Ok(notes)
}

#[tauri::command]
async fn update_note(state: tauri::State<'_, Mutex<AppState>>, note: Note) -> Result<(), String> {
    println!("update_note()");

    let app_state = state.lock().await;
    let db = &<std::option::Option<Pool<Postgres>> as Clone>::clone(&app_state.db)
        .ok_or("DB not seted")?;

    sqlx::query("UPDATE notes SET description = $1, status = $2 WHERE id = $3")
        .bind(note.description)
        .bind(note.status)
        .bind(note.id)
        .execute(db)
        .await
        .map_err(|e| {
            println!("could not update Note {}", e);
            format!("could not update Note {}", e)
        })?;

    Ok(())
}

#[tauri::command]
async fn delete_note(state: tauri::State<'_, Mutex<AppState>>, id: i32) -> Result<(), String> {
    println!("delete_note() id = {}", id);

    let app_state = state.lock().await;
    let db = &<std::option::Option<Pool<Postgres>> as Clone>::clone(&app_state.db)
        .ok_or("DB not seted")?;

    sqlx::query("DELETE FROM notes WHERE id = $1")
        .bind(id)
        .execute(db)
        .await
        .map_err(|e| format!("could not delete note {}", e))?;

    Ok(())
}

#[tauri::command]
fn get_app_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}
