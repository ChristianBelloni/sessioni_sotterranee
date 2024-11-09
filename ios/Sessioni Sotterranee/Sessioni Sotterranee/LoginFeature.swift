//
//  LoginFeature.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 05/11/24.
//

import Foundation
import SwiftUI
import ComposableArchitecture

@Reducer
struct LoginFeature {
    
    @Dependency(\.apiClient) var apiClient
    
    @ObservableState
    struct State: Equatable {
        @Shared var user: User?
        var requiresUsername: Bool = false
        var newUsername: String = ""
        var isLoading = false
    }
    
    enum Action {
        case StartLogin
        case Loading
        case RequiresUsername
        case UsernameChanged(String)
        case SetUsername
        case LoggedIn
    }
    
    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .StartLogin:
                return .run { send in
                    try await apiClient.signIn()
                    let user = try await apiClient.getUser()
                    if user.username != "" {
                        await send(.LoggedIn)
                    } else {
                        await send(.RequiresUsername)
                    }
                    
                }
            case .RequiresUsername:
                state.requiresUsername = true
                return .none
            case .UsernameChanged(let username):
                state.newUsername = username
                return .none
            case .SetUsername:
                return .run { [username = state.newUsername] send in
                    try await apiClient.setUsername(username: username)
                    await send(.LoggedIn)
                }
            case .LoggedIn:
                return .none
            case .Loading:
                state.isLoading = true
                return .none
            }
        
        }
    }
}

struct LoginView: View {
    @Bindable var store: StoreOf<LoginFeature>
    
    var body: some View {
        Group {
            if store.isLoading {
                ProgressView().progressViewStyle(.circular)
            } else {
                VStack {
                    if store.requiresUsername {
                        TextField("username", text: $store.newUsername.sending(\.UsernameChanged))
                        Button("Conferma") {
                            store.send(.SetUsername)
                        }
                    } else {
                        Button("Login") {
                            store.send(.StartLogin)
                        }
                    }
                }
            }
        }
    }
}
