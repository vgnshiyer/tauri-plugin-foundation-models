use serde::de::DeserializeOwned;
use tauri::{plugin::PluginApi, AppHandle, Runtime};

use crate::models::*;

pub fn init<R: Runtime, C: DeserializeOwned>(
  app: &AppHandle<R>,
  _api: PluginApi<R, C>,
) -> crate::Result<FoundationModels<R>> {
  Ok(FoundationModels(app.clone()))
}

/// Access to the foundation-models APIs.
pub struct FoundationModels<R: Runtime>(AppHandle<R>);

impl<R: Runtime> FoundationModels<R> {
  pub fn ping(&self, payload: PingRequest) -> crate::Result<PingResponse> {
    Ok(PingResponse {
      value: payload.value,
    })
  }
}
