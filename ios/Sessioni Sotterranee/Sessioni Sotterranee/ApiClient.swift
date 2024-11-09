//
//  ApiClient.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 04/11/24.
//

import Foundation
import ComposableArchitecture
import OpenAPIURLSession
import Logto
import LogtoClient

struct ApiClient {
    var user: Shared<User?> = Shared(nil)
    var client: any APIProtocol
    var authClient: any AuthService
    
    init(serverURL: URL, redirectUrl: String) {
        let authClient = AuthClient(
            redirectUrl: redirectUrl,
            client: LogtoClient(
                useConfig: try! LogtoConfig(
                    endpoint: "https://auth-dev.sessioni-sotterranee.info/",
                    appId: "virc2uruta8tetclpuu03"
                )
            )
        )
        self.authClient = authClient
        self.client = Client(serverURL: serverURL, transport: URLSessionTransport(), middlewares: [AuthMiddleware(inner: authClient)])
    }
}

protocol ApiClientProtocol: AuthClientProtocol {
    var user: Shared<User?> { get }
    func getUser() async throws -> User
    func setUsername(username: String) async throws
    func loadEvents(limit: Int, offset: Int) async throws -> [Event]
}

extension ApiClient: ApiClientProtocol {
    func loadEvents(limit: Int, offset: Int) async throws -> [Event] {
        let events = try await self.client.get_sol_api_sol_events_sol_(query: .init(limit: limit, offset: offset)).ok.body.json
        
        return events.map({ Event($0) })
    }
    
    func signIn() async throws {
        try await self.authClient.signIn()
        if let user = try? await self.getUser() {
            await self.user.withLock({ inner in inner = user})
        }
    }
    
    func signOut() async {
        await self.authClient.signOut()
        await self.user.withLock({ inner in inner = nil })
    }
    
    
    func getUser() async throws -> User {
        let userData = try await self.client.get_sol_api_sol_users_sol_me().ok.body.json
        let user = User(id: Int(userData.id), logtoId: userData.log_to_id, username: userData.username)
        await self.user.withLock({ inner in inner = user })
        return user
    }
    
    func setUsername(username: String) async throws {
        _ = try await self.client.patch_sol_api_sol_users_sol_set_username(query: .init(username: username)).ok
        let user = try await getUser()
        await self.user.withLock({ inner in inner = user })
    }
}

fileprivate enum ApiClientKey : DependencyKey {
    static let liveValue: any ApiClientProtocol = ApiClient(serverURL: URL(string: "http://localhost:8080")!, redirectUrl: "io.logto://callback")
}

extension DependencyValues {
    var apiClient: ApiClientProtocol {
        get { self[ApiClientKey.self] }
        set { self[ApiClientKey.self] = newValue }
      }
}
