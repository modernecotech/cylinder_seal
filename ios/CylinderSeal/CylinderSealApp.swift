import BackgroundTasks
import SwiftUI

/// Application entrypoint.
///
/// Creates the dependency graph at boot and injects it into the SwiftUI
/// scene via `.environmentObject`. Also registers the background sync task
/// identifier so `BGTaskScheduler` can fire it.
@main
struct CylinderSealApp: App {
    @UIApplicationDelegateAdaptor(AppDelegate.self) private var appDelegate
    @StateObject private var container: AppContainer

    init() {
        _container = StateObject(wrappedValue: AppContainer.bootstrap())
    }

    var body: some Scene {
        WindowGroup {
            ContentView()
                .environmentObject(container)
                .environmentObject(container.walletKeyManager)
                .environmentObject(container.userPreferences)
                .environmentObject(container.database)
                .environmentObject(container.ingestor)
                .environmentObject(container.bleService)
                .environmentObject(container.nfcReader)
                .environmentObject(container.syncClient)
        }
    }
}

/// UIKit glue. Claims the BGTask identifiers and forwards scheduling to
/// `SyncWorker`.
final class AppDelegate: NSObject, UIApplicationDelegate {
    func application(
        _ application: UIApplication,
        didFinishLaunchingWithOptions launchOptions: [UIApplication.LaunchOptionsKey: Any]? = nil
    ) -> Bool {
        SyncWorker.registerTasks()
        return true
    }

    func applicationDidEnterBackground(_ application: UIApplication) {
        SyncWorker.schedule()
    }
}
