//
//  AuthClient.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 31/10/24.
//

import Foundation
import ComposableArchitecture
import Logto
import LogtoClient

struct LogtoAuthClient: AuthClient {
    let client: LogtoClient
    
    init() {
        client = LogtoClient(useConfig: try! .init(endpoint: "https://auth-dev.sessioni-sotterranee.info/", appId: "virc2uruta8tetclpuu03"))
    }
    
    var isAuthenticated: Bool {
        client.isAuthenticated
    }
    
    func signIn() async throws {
        try await client.signInWithBrowser(redirectUri: "io.logto://callback")
    }
    
    func signOut() async {
        await client.signOut()
    }
}

protocol AuthClient {
    var isAuthenticated: Bool { get }
    func signIn() async throws
    func signOut() async
}

fileprivate enum AuthClientKey: DependencyKey {
    static let liveValue: any AuthClient = LogtoAuthClient()
}

extension DependencyValues {
    var authClient: AuthClient {
        get { self[AuthClientKey.self] }
        set { self[AuthClientKey.self] = newValue }
      }
}
