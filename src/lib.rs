use tauri::{
  plugin::{Builder, TauriPlugin},
  Manager, Runtime,
};

pub use models::*;

#[cfg(desktop)]
mod desktop;
#[cfg(mobile)]
mod mobile;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

#[cfg(desktop)]
use desktop::FoundationModels;
#[cfg(mobile)]
use mobile::FoundationModels;

/// Extensions to [`tauri::App`], [`tauri::AppHandle`] and [`tauri::Window`] to access the foundation-models APIs.
pub trait FoundationModelsExt<R: Runtime> {
  fn foundation_models(&self) -> &FoundationModels<R>;
}

impl<R: Runtime, T: Manager<R>> crate::FoundationModelsExt<R> for T {
  fn foundation_models(&self) -> &FoundationModels<R> {
    self.state::<FoundationModels<R>>().inner()
  }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new("foundation-models")
    .invoke_handler(tauri::generate_handler![commands::ping])
    .setup(|app, api| {
      #[cfg(mobile)]
      let foundation_models = mobile::init(app, api)?;
      #[cfg(desktop)]
      let foundation_models = desktop::init(app, api)?;
      app.manage(foundation_models);
      Ok(())
    })
    .build()
}
