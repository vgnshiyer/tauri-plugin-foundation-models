import Foundation
import FoundationModels
import SwiftRs

@available(macOS 26.0, iOS 26.0, *)
@_cdecl("check_availability_swift")
public func checkAvailabilitySwift() -> SRString {
    let model = SystemLanguageModel.default
    let status: String
    
    switch model.availability {
    case .available:
        status = "available"
    case .unavailable(.deviceNotEligible):
        status = "deviceNotEligible"
    case .unavailable(.appleIntelligenceNotEnabled):
        status = "appleIntelligenceNotEnabled"
    case .unavailable(.modelNotReady):
        status = "modelNotReady"
    case .unavailable(_):
        status = "unknown"
    @unknown default:
        status = "unknown"
    }
    
    return SRString(status)
}
