use tauri::{command, AppHandle, Runtime};

use crate::models::*;
use crate::FoundationModelsExt;
use crate::Result;

#[command]
pub(crate) async fn check_availability<R: Runtime>(
    app: AppHandle<R>,
) -> Result<AvailabilityResponse> {
    app.foundation_models().check_availability()
}
