use crate::models::*;
use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<FoundationModels<R>> {
    Ok(FoundationModels(app.clone()))
}

pub struct FoundationModels<R: Runtime>(AppHandle<R>);

impl<R: Runtime> FoundationModels<R> {
    pub fn check_availability(&self) -> crate::Result<AvailabilityResponse> {
        Ok(AvailabilityResponse {
            status: "unavailable".to_string(),
        })
    }
}
