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

protocol AuthMiddlewareProtocol: Sendable {
    func jwtHeader() async throws -> String?
}

struct AuthMiddleware<Inner> {
    let inner: Inner
}

extension AuthMiddleware: ClientMiddleware where Inner == AuthMiddlewareProtocol {
    func intercept(_ request: HTTPTypes.HTTPRequest, body: OpenAPIRuntime.HTTPBody?, baseURL: URL, operationID: String, next: @Sendable (HTTPTypes.HTTPRequest, OpenAPIRuntime.HTTPBody?, URL) async throws -> (HTTPTypes.HTTPResponse, OpenAPIRuntime.HTTPBody?)) async throws -> (HTTPTypes.HTTPResponse, OpenAPIRuntime.HTTPBody?) {
        var request = request
        
        if let authHeader = try? await self.inner.jwtHeader() {
            request.headerFields[.authorization] = authHeader
        }
        return try await next(request, body, baseURL)
    }
}

protocol AuthClientProtocol  {
    func signIn() async throws
    func signOut() async
}

struct AuthClient {
    let redirectUrl: String
    let client: LockIsolated<LogtoClient>
    
    init(redirectUrl: String, client: LogtoClient) {
        self.redirectUrl = redirectUrl
        self.client = .init(client)
    }
}

protocol AuthService: AuthClientProtocol, AuthMiddlewareProtocol { }

extension AuthClient: AuthService {
    func signIn() async throws {
        try await self.client.value.signInWithBrowser(redirectUri: self.redirectUrl)
    }
    
    func signOut() async {
        await self.client.value.signOut()
    }
    
    func jwtHeader() async throws -> String? {
        if let authHeader = self.client.idToken {
            return "Bearer \(authHeader)"
        } else {
            // TODO: implement an error type
            return nil
        }
    }
}

