#![cfg(any(target_os = "ios", target_os = "macos"))]

use serde::de::DeserializeOwned;
use swift_rs::{swift, SRString};
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

swift!(fn check_availability_swift() -> SRString);

pub fn init<R: Runtime, C: DeserializeOwned>(
    app: &AppHandle<R>,
    _api: PluginApi<R, C>,
) -> crate::Result<FoundationModels<R>> {
    Ok(FoundationModels(app.clone()))
}

/// Access to the foundation-models APIs.
pub struct FoundationModels<R: Runtime>(AppHandle<R>);

impl<R: Runtime> FoundationModels<R> {
    pub fn check_availability(&self) -> crate::Result<AvailabilityResponse> {
        let status = unsafe { check_availability_swift() };
        Ok(AvailabilityResponse {
            status: status.as_str().to_string(),
        })
    }
}
