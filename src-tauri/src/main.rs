#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use std::{error::Error, path::PathBuf};
use log::error;

use plasmo_launcher::{task::{queue::TaskQueue, tasks::{download_meta::DownloadMetaTask, download_java::DownloadJavaTask, download_libraries::DownloadLibrariesTask, launch_game::LaunchGameTask, download_assets::DownloadAssetsTask, cache_session_skin::{cache_skin_nick, update_skin}, cache_meta_instance::CacheMetaInstanceTask, download_meta_instance::DownloadMetaInstanceTask}, progress::NamedProgress}, state::{config::LauncherConfig, session::{Session, SessionState}, preferences::{Preferences, MemoryAllocation}}, utils::tauri_block_on::tauri_block_on, external_api::{plasmo::{models::{user::User, error::PlasmoError}}, metaserver::meta_instance::{get_meta_instance, try_get_meta_instance}}, instance::InstanceTrait};
use plasmolauncher_common::models::metaserver::meta_instance::MetaInstance;
// use plasmo_launcher::{task::{queue::TaskQueue, tasks::{download_java::DownloadJavaTask, download_meta::{DownloadMetaTask}, download_libraries::DownloadLibrariesTask, download_assets::DownloadAssetsTask, launch_game::{LaunchGameTask}}}, utils::pathbuf_to_string::ToOptionString, config::LauncherConfig}, session::Session, preferences::Preferences};
use tauri::{State, Manager, AppHandle, api::shell::open, WindowBuilder, WindowUrl, LogicalSize};
use tauri_plugin_synced_state::{SyncStateToml, SyncedToml, SyncState};
use tokio::fs::create_dir_all;
use tauri_plugin_log::{LogTarget, Builder};

const INSTANCE_NAME: &'static str = "main";

#[tauri::command]
async fn start_task(queue: State<'_, TaskQueue>, handle: AppHandle, _config: State<'_, LauncherConfig>) -> Result<(), ()> {

  queue.add_task(CacheMetaInstanceTask::new(INSTANCE_NAME)).await;
  queue.start_queue().await;

  let instance = match get_meta_instance(INSTANCE_NAME, &handle).await {
    Ok(value) => value,
    Err(error) => {
      handle.emit_all("global-error", error.to_string()).ok();
      return Err(())
    }
  }.instance;

  queue.add_task(DownloadMetaTask::new(instance.clone())).await;
  queue.add_task(DownloadJavaTask::new(instance.clone())).await;
  queue.add_task(DownloadLibrariesTask::new(instance.clone(), false)).await;
  queue.add_task(DownloadAssetsTask::new(instance.clone(), false)).await;
  queue.add_task(DownloadMetaInstanceTask::new(INSTANCE_NAME)).await;
  queue.add_task(LaunchGameTask::new(instance.clone())).await;
  queue.start_queue().await;

  Ok(())
}

#[tauri::command]
async fn get_queue_processing(queue: State<'_, TaskQueue>) -> Result<bool, ()> {
  Ok(queue.get_processing().await)
}

#[tauri::command]
async fn set_offline_session(
  nick: String,
  session: SyncStateToml<'_, Session>,
  handle: AppHandle
) -> Result<(), ()> {

  session.set_offline_session(&nick).await;

  if let Err(error) = cache_skin_nick(handle, &nick).await {
    error!("Error while caching skin: {error}");
  }

  Ok(())
}

#[tauri::command]
async fn set_memory_alloc(
  value: MemoryAllocation,
  prefs: SyncStateToml<'_, Preferences>
) -> Result<(), ()> {
  prefs.mutate(|prefs| prefs.java.maximum_memory_allocation = value).await;
  Ok(())
}

#[tauri::command]
async fn set_optional_mod(
  value: bool,
  id: String,
  prefs: SyncStateToml<'_, Preferences>
) -> Result<(), ()> {
  prefs.mutate(|prefs| {
    prefs.optional_mods.insert(id, value);
  }).await;
  Ok(())
}

#[tauri::command]
async fn set_plasmo_session(
  token: String,
  session: SyncStateToml<'_, Session>,
  handle: AppHandle
) -> Result<(), PlasmoError> {

  let user = User::from_token(&handle, &token).await?;

  let nick = user.get_nick()?;

  session.set_plasmo_session(&nick, &token).await;
  
  if let Err(error) = cache_skin_nick(handle, &nick).await {
    error!("Error while caching skin: {error}");
  }

  Ok(())
}

#[tauri::command]
async fn get_session(session: SyncStateToml<'_, Session>) -> Result<Session, ()> {
  Ok(session.get().await)
}

#[tauri::command]
async fn get_prefs(prefs: SyncStateToml<'_, Preferences>) -> Result<Preferences, ()> {
  Ok(prefs.get().await)
}


#[tauri::command]
async fn get_progress(progress: SyncState<'_, NamedProgress>) -> Result<NamedProgress, ()> {
  Ok(progress.get().await)
}

#[tauri::command]
async fn update_session(handle: AppHandle, session_state: SyncStateToml<'_, Session>) -> Result<(), PlasmoError> {

  let session = session_state.get().await;

  let Session::Plasmo(plasmo_session) = session else { return Ok(()) };

  let token = plasmo_session.token;

  let user = User::from_token(&handle, &token).await?;

  let nick = user.get_nick()?;

  if &nick == &plasmo_session.nick { return Ok(()) };

  session_state.set_plasmo_session(&nick, &token).await;

  if let Err(error) = cache_skin_nick(handle, &nick).await {
    error!("Error while caching skin: {error}");
  }

  Ok(())
}

#[tauri::command]
async fn open_instances_folder(handle: AppHandle) -> Result<(), ()> {

  let instance = match try_get_meta_instance(INSTANCE_NAME, &handle).await {
    Ok(value) => value,
    Err(error) => {
      handle.emit_all("global-error", error.to_string()).ok();
      return Err(())
    }
  }.instance;
  
  let Ok(path) = instance.get_dir_path(&handle) else {
    return Err(())
  };

  create_dir_all(&path).await.ok();

  open(
    &handle.shell_scope(),
    path.to_str().unwrap(),
    None
  ).ok();

  Ok(())
}

#[tauri::command]
async fn open_plasmo_oauth(handle: AppHandle, config: State<'_, LauncherConfig>) -> Result<(), ()> {

  let mut url = config.plasmo.oauth_url.clone();

  url.query_pairs_mut()
    .append_pair("client_id", &config.plasmo.client_id)
    .append_pair("redirect_uri", "https://rp.plo.su/oauth2_redirect")
    .append_pair("response_type", "token")
    .append_pair("scope", "mc:auth");

  open(
    &handle.shell_scope(),
    url.as_str(),
    None
  ).ok();

  Ok(())
}

#[tauri::command(async)]
fn set_height(height: u32, window: tauri::Window) -> Option<()> {
  let scale_factor = window.scale_factor().ok()?;
  let current_size = window
    .inner_size()
    .ok()?
    .to_logical::<u32>(scale_factor);
  window.set_size(LogicalSize::new(current_size.width, height)).ok()?;
  // dbg!(result);
  Some(())
}

#[tauri::command]
async fn check_skin_update(
  handle: AppHandle
) -> Result<bool, ()> {

  let check = update_skin(handle).await.unwrap_or(false);

  Ok(check)
}

#[tauri::command]
async fn get_meta_instance_cache(handle: AppHandle) -> Option<MetaInstance> {
  try_get_meta_instance(INSTANCE_NAME, handle).await.ok()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

  tauri::Builder::default()
    .plugin(tauri_plugin_synced_state::PluginBuilder::new()
      .manage_toml::<Session>("session", "session.toml")
      .manage_toml::<Preferences>("prefs", "prefs.toml")
      .manage::<NamedProgress>("progress")
      .build()
    )
    .plugin(Builder::default().targets([
      LogTarget::LogDir,
      LogTarget::Stdout,
    ]).build())
    .manage(LauncherConfig::from_file())
    .invoke_handler(tauri::generate_handler![
      start_task,
      get_queue_processing,
      set_offline_session,
      open_instances_folder,
      get_session,
      get_prefs,
      set_height,
      check_skin_update,
      open_plasmo_oauth,
      set_memory_alloc,
      set_optional_mod,
      set_plasmo_session,
      update_session,
      get_progress,
      get_meta_instance_cache
    ])
    .setup(|app| {

      let handle = app.handle();

      app.manage(TaskQueue::new(handle));

      let session = tauri_block_on(
        app.state::<SyncedToml<Session>>().get()
      );

      let mut url = PathBuf::new();

      match session {
        Session::None => {},
        _ => url.push("launcher"),
      }

      let config = app.state::<LauncherConfig>();

      WindowBuilder::new(app, "main", WindowUrl::App(url))
        .visible(false)
        .resizable(false)
        .title(&config.window_title)
        .min_inner_size(0., 0.)
        .inner_size(440., 440.)
        .build()
        .expect("Failed to create main window");
        
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  Ok(())
}
