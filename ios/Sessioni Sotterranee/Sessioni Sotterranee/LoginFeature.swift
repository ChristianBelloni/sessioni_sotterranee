//
//  LoginFeature.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 31/10/24.
//

import Foundation
import SwiftUI
import ComposableArchitecture

@Reducer
struct LoginFeature {
    @Dependency(\.authClient) var authClient
    @Dependency(\.apiClient) var apiClient
    
    @ObservableState
    struct State: Equatable {
        var requiresUsername: Bool = false
        var username: String = ""
        var error: String?
    }
    
    enum Action {
        case LaunchLogin
        case LoggedIn(User)
        case RequiresUsername
        case UsernameChanged(String)
        case SetUsername
        case Error(String)
    }
    
    var body: some Reducer<State, Action> {
        Reduce { state, action in
            state.error = nil
            switch action {
            case .LaunchLogin:
                return .run { send in
                    do {
                        try await authClient.signIn()
                        if (await authClient.username()) != nil {
                            let user = try await apiClient.get_sol_api_sol_users_sol_me().ok.body.json
                            
                            await send(.LoggedIn(User(id: Int(user.id), logtoId: user.log_to_id, username: user.username)))
                        } else {
                            await send(.RequiresUsername)
                        }
                    } catch {
                        await send(.Error(error.localizedDescription))
                    }
                }
            case .Error(let error):
                state.error = error
            case .RequiresUsername:
                state.requiresUsername = true
            case .UsernameChanged(let newValue):
                state.username = newValue
            case .SetUsername:
                return .run { [username = state.username] send in
                    do {
                        _ = try await apiClient.patch_sol_api_sol_users_sol_set_username(query: .init(username: username)).ok
                        let user = try await apiClient.get_sol_api_sol_users_sol_me().ok.body.json
                        
                        await send(.LoggedIn(User(id: Int(user.id), logtoId: user.log_to_id, username: user.username)))
                    } catch {
                        print(error.localizedDescription)
                    }
                    
                }
            default:
                return .none
            }
            return .none
        }
    }
}

struct LoginView : View {
    @Dependency(\.authClient) var authClient
    @Dependency(\.apiClient) var apiClient
    @Bindable var store: StoreOf<LoginFeature>
    
    var body: some View {
        ZStack {
            LinearGradient(colors: [Color.accentColor, Color.text], startPoint: .bottom, endPoint: .top)
            if store.state.requiresUsername {
                VStack {
                    TextField("Sessioni Sotterranee", text: $store.username.sending(\.UsernameChanged))
                        .font(.largeTitle)
                        .foregroundStyle(.white.opacity(0.8))
                        .padding(20)
                        .frame(maxWidth: .infinity)
                        .background(.secondary)
                        .clipShape(.buttonBorder)
                    Spacer()
                    Button("Prosegui") {
                        store.send(.SetUsername)
                    }
                    .foregroundStyle(.white.opacity(0.8))
                    .padding()
                    .frame(maxWidth: .infinity)
                    .font(.title)
                    .background(.secondary)
                    .clipShape(.buttonBorder)
                    
                }.padding(.vertical, 150)
                    .padding(.horizontal, 20)
            } else {
                VStack {
                    Text("Sessioni Sotterranee")
                        .font(.largeTitle)
                        .foregroundStyle(.white.opacity(0.8))
                        .padding(20)
                        .frame(maxWidth: .infinity)
                        .background(.secondary)
                        .clipShape(.buttonBorder)
                    Spacer()
                    Button("Inizia") {
                        store.send(.LaunchLogin)
                    }
                    .foregroundStyle(.white.opacity(0.8))
                    .padding()
                    .frame(maxWidth: .infinity)
                    .font(.title)
                    .background(.secondary)
                    .clipShape(.buttonBorder)
                }.padding(.vertical, 150)
                    .padding(.horizontal, 20)
            }
        }.onAppear() {
            Task {
                
                if authClient.isAuthenticated {
                    if (await authClient.username()) != nil {
                        let user = try await apiClient.get_sol_api_sol_users_sol_me().ok.body.json
                        
                        store.send(.LoggedIn(User(id: Int(user.id), logtoId: user.log_to_id, username: user.username)))
                        
                    } else {
                        store.send(.RequiresUsername)
                    }
                }
                
            }
        }
    }
}
#Preview{
    LoginView(store: Store(initialState: .init()) {
        LoginFeature()
    })
}
