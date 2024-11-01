//
//  Sessioni_SotterraneeApp.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 31/10/24.
//

import SwiftUI
import ComposableArchitecture

@main
struct Sessioni_SotterraneeApp: App {
    @Dependency(\.authClient) var authClient
    
    var body: some Scene {
        WindowGroup {
            ZStack {
                LinearGradient(colors: [.black, .gray], startPoint: .top, endPoint: .bottom)
                AppView(store: Store(initialState: .init(isAuthenticated: authClient.isAuthenticated)) {
                    AppFeature()
                })
            }.ignoresSafeArea()
        }
    }
}
