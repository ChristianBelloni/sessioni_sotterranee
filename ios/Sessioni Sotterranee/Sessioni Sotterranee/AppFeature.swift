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
struct AppFeature {
    @Dependency(\.authClient) var authClient
    
    @ObservableState
    struct State: Equatable {
        var authState: AuthState
        var login: LoginFeature.State
        var home: HomeFeature.State
        init() {
            let login: LoginFeature.State = .init()
            let home: HomeFeature.State = .init()
            self.authState = .NotLogged(.init())
            self.login = login
            self.home = home
            
        }
    }
    
    @ObservableState
    enum AuthState: Equatable {
        case Logged(HomeFeature.State)
        case NotLogged(LoginFeature.State)
    }
    
    enum Action {
        case loginFeature(LoginFeature.Action)
        case homeFeature(HomeFeature.Action)
    }
    
    var body: some Reducer<State, Action> {
        Scope(state: \.login, action: \.loginFeature) {
            LoginFeature()
        }
        Scope(state: \.home, action: \.homeFeature) {
            HomeFeature()
        }
        
        Reduce { state, action in
            switch action {
            case .loginFeature(let action):
                switch action {
                case .LoggedIn:
                    state.authState = .Logged(.init())
                default:
                    return .none
                }
            case .homeFeature(let action):
                switch action {
                case .Logout:
                    state.authState = .NotLogged(.init())
                    return .none
                default:
                    return .none
                }
            }
            return .none
        }
    }
    
    
}

struct AppView: View {
    
    let store: StoreOf<AppFeature>
    
    var body: some View {
        switch store.authState {
        case .Logged(let state):
            HomeView(store: store.scope(state: \.home, action: \.homeFeature))
        case .NotLogged(let state):
            LoginView(store: store.scope(state: \.login, action: \.loginFeature))
        }
    }
}

#Preview {
    AppView(store: Store(initialState: .init()) {
        AppFeature()
    })
}
