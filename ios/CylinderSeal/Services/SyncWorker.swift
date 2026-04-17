import BackgroundTasks
import Foundation
import UIKit

/// Background sync using `BGTaskScheduler`.
///
/// Register the task identifier at launch (via `registerTasks()`), schedule
/// it when the app moves to background (via `schedule()`). iOS decides when
/// to actually run it; typical windows are a few minutes of wall-time but
/// sometimes less. The worker drains the pending queue once per execution.
enum SyncWorker {
    static let identifier = "iq.cbi.cylinderseal.sync"

    @MainActor
    static func registerTasks() {
        BGTaskScheduler.shared.register(
            forTaskWithIdentifier: identifier,
            using: nil
        ) { task in
            guard let task = task as? BGProcessingTask else { return }
            Task { await handle(task: task) }
        }
    }

    @MainActor
    static func schedule() {
        let request = BGProcessingTaskRequest(identifier: identifier)
        request.requiresNetworkConnectivity = true
        request.requiresExternalPower = false
        request.earliestBeginDate = Date(timeIntervalSinceNow: 15 * 60)
        do {
            try BGTaskScheduler.shared.submit(request)
        } catch {
            NSLog("BGTaskScheduler submit failed: \(error)")
        }
    }

    @MainActor
    private static func handle(task: BGProcessingTask) async {
        // Reschedule first so we stay on the queue even if something fails.
        schedule()

        task.expirationHandler = {
            NSLog("sync task expired")
        }

        // Pull the singleton sync client out of the scene's container. We
        // reach into the window scene to avoid duplicating the gRPC stack.
        guard
            let scene = UIApplication.shared.connectedScenes.first as? UIWindowScene,
            let root = scene.windows.first?.rootViewController,
            let client = ChainSyncLocator.locate(from: root)
        else {
            task.setTaskCompleted(success: false)
            return
        }

        await client.drainOnce()
        task.setTaskCompleted(success: true)
    }
}

/// Small helper that walks the SwiftUI environment to find the live
/// `ChainSyncClient`. SwiftUI doesn't expose `@EnvironmentObject`s on the
/// root view controller directly; this hook expects the app to stash a
/// reference via `setCurrent(_:)` when the scene activates.
enum ChainSyncLocator {
    @MainActor private static var shared: ChainSyncClient?

    @MainActor static func setCurrent(_ client: ChainSyncClient) { shared = client }
    @MainActor static func locate(from _: UIViewController) -> ChainSyncClient? { shared }
}
