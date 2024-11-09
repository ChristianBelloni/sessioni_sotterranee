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
    @Dependency(\.apiClient) var apiClient
    
    var body: some Scene {
        WindowGroup {
            RootView(store: Store(initialState: RootFeature.initialState(user: apiClient.user)) {
                RootFeature()
            })
            .ignoresSafeArea().task {
                do {
                    _ = try await apiClient.getUser()
                } catch { }
            }
        }
    }
}
