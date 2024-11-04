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
import OpenAPIRuntime
import OpenAPIURLSession
import HTTPTypes

struct LogtoAuthClient: AuthClient {
    func intercept(_ request: HTTPTypes.HTTPRequest, body: OpenAPIRuntime.HTTPBody?, baseURL: URL, operationID: String, next: @Sendable (HTTPTypes.HTTPRequest, OpenAPIRuntime.HTTPBody?, URL) async throws -> (HTTPTypes.HTTPResponse, OpenAPIRuntime.HTTPBody?)) async throws -> (HTTPTypes.HTTPResponse, OpenAPIRuntime.HTTPBody?) {
        
        var request = request
        
                // Adds the `Authorization` header field with the provided value.
        let value = try! await self.client.idToken!
        
        let token = try! self.client.getIdTokenClaims()
            request.headerFields[.authorization] = "Bearer \(value)"
            do{
                let result = try await URLSession.shared.data(for: URLRequest(url: URL(string: "https://lecl3f.logto.app/api")!))
            } catch {
                print(error.localizedDescription)
            }
            
        
        return try await next(request, body, baseURL)
    }
    
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
    
    func username() async -> String? {
        return try? await client.fetchUserInfo().username
    }
}

protocol AuthClient : ClientMiddleware{
    var isAuthenticated: Bool { get }
    func signIn() async throws
    func signOut() async
    func username() async -> String?
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
