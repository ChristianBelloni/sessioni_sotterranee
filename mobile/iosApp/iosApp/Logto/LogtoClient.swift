//
//  LogtoClient.swift
//  iosApp
//
//  Created by Christian Belloni on 08/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//
import Foundation
import Shared
import Logto
import LogtoClient

class AuthClient {
    let redirectUrL: String
    let client: LogtoClient
    
    init(redirectUrL: String, client: LogtoClient) {
        self.redirectUrL = redirectUrL
        self.client = client
    }
}

extension AuthClient : Shared.LogtoClientImpl {
    public func signIn() async throws {
        try await self.client.signInWithBrowser(redirectUri: self.redirectUrL)
    }
    
    public func signOut() async throws {
        await self.client.signOut()
    }
    
    public func user() async throws -> LogtoUser {
        let user = try await self.client.fetchUserInfo()
        return LogtoUser(sub: user.sub, username: user.username)
    }
    
    public func bearerToken() async throws -> String? {
        if let tkn = self.client.idToken {
            "Bearer \(tkn)"
        } else {
            nil
        }
    }
}
