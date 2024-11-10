import SwiftUI
import Shared
import Logto
import LogtoClient

@main
struct iOSApp: App {
    let apiClient = AuthClient(
        redirectUrL: "io.logto://callback",
        client: try! LogtoClient(
            useConfig: LogtoConfig(
                endpoint: "https://auth-dev.sessioni-sotterranee.info/",
                appId: "virc2uruta8tetclpuu03"
            )
        )
    )
    var body: some Scene {
        WindowGroup {
            RootView(
                appViewModel: RootViewModel(apiClient: ApiService(authClient: apiClient))
            ).ignoresSafeArea()
        }
    }
}
