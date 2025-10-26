import Foundation
import SwiftRs
import Tauri
import UIKit
import WebKit

class FoundationModelsPlugin: Plugin {
  @objc public func (_ invoke: Invoke) throws {
    let model = SystemLanguageModel.default
    switch model.availability {
      case .available:
        invoke.resolve(["value": "available"])
      case .unavailable(.deviceNotEligible):
        invoke.resolve(["value": "device not eligible"])
    }
  }
}

@_cdecl("init_plugin_foundation_models")
func initPlugin() -> Plugin {
  return FoundationModelsPlugin()
}
