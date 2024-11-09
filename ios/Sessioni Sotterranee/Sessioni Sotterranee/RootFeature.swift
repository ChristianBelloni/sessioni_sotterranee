//
//  ContentView.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 31/10/24.
//

import SwiftUI
import SwiftData
import ComposableArchitecture


@Reducer
struct RootFeature {
    @Dependency(\.apiClient) var apiClient
    
    @ObservableState
    struct State {
        @Shared var user: User?
        var login: LoginFeature.State
        var app: AppFeature.State
    }
    
    enum Action {
        case login(LoginFeature.Action)
        case app(AppFeature.Action)
    }
    
    var body: some ReducerOf<Self> {
        Scope(state: \.login, action: \.login) {
            LoginFeature()
        }
        Scope(state: \.app, action: \.app) {
            AppFeature()
        }
        
        Reduce { state, action in
            return .none
        }
    }
    
    static func initialState(user: Shared<User?>) -> Self.State {
        .init(user: user, login: .init(user: user), app: AppFeature.initialState(user: user))
    }
}

struct RootView: View {
    @Dependency(\.websocketClient) var websocketClient
    
    let store: StoreOf<RootFeature>
    
    var body: some View {
        if let user = store.user {
            AppView(store: store.scope(state: \.app, action: \.app)).onAppear() {
                websocketClient.subscribe { @MainActor msg in
                    store.send(.app(.mainChat(.MessageReceived(msg))))
                }
            }
        } else {
            LoginView(store: store.scope(state: \.login, action: \.login))
        }
    }
}
