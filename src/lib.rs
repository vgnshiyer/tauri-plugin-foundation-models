use tauri::{
    plugin::{Builder, TauriPlugin},
    Manager, Runtime,
};

pub use models::*;

// iOS and macOS implementation
#[cfg(any(target_os = "ios", target_os = "macos"))]
mod plugin;

// Stub implementation for other platforms
#[cfg(not(any(target_os = "ios", target_os = "macos")))]
mod unsupported;

mod commands;
mod error;
mod models;

pub use error::{Error, Result};

use plugin::FoundationModels;

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
        .invoke_handler(tauri::generate_handler![commands::check_availability])
        .setup(|app, api| {
            let foundation_models = plugin::init(app, api)?;
            app.manage(foundation_models);
            Ok(())
        })
        .build()
}
