import SwiftUI
import Shared
import Logto
import LogtoClient

@main
struct iOSApp: App {
    var body: some Scene {
        WindowGroup {
            AppView(
                appViewModel: AppViewModel(
                    loginViewModel: LoginViewModel(
                        authClient: AuthClient(
                            redirectUrL: "io.logto://callback",
                            client: try! LogtoClient(
                                useConfig: LogtoConfig(
                                    endpoint: "https://auth-dev.sessioni-sotterranee.info/",
                                    appId: "virc2uruta8tetclpuu03"
                                )
                            )
                        )
                    )
                )
            )
        }
    }
}
