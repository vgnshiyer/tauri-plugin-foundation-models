import { invoke } from '@tauri-apps/api/core';

export type AvailabilityStatus = 
  | 'available'
  | 'deviceNotEligible'
  | 'appleIntelligenceNotEnabled'
  | 'modelNotReady'
  | 'unknown'
  | 'unavailable';

export interface AvailabilityResponse {
  status: AvailabilityStatus;
}

export async function checkAvailability(): Promise<AvailabilityResponse> {
  return await invoke<AvailabilityResponse>(
    'plugin:foundation-models|check_availability'
  );
}
