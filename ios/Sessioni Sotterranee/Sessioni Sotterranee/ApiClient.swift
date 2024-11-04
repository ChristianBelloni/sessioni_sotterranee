//
//  ApiClient.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 04/11/24.
//

import Foundation
import ComposableArchitecture
import OpenAPIURLSession




extension DependencyValues {
    var apiClient: APIProtocol {
        get { Client(serverURL: URL(string: "http://localhost:8080")!, transport: URLSessionTransport(), middlewares: [self.authClient]) }
        set {  }
      }
}
