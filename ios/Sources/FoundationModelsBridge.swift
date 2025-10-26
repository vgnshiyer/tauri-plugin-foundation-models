import Foundation
import FoundationModels
import SwiftRs

@_cdecl("check_availability_swift")
public func checkAvailabilitySwift() -> SRString {
    let status: String

    if #available(iOS 26, macOS 26, *) {
        let model = SystemLanguageModel.default
        
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
    } else {
        // Fallback for older OS versions
        status = "notSupported"
    }

    return SRString(status)
}
