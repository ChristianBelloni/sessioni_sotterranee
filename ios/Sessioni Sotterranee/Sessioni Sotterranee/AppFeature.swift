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
        
        @Shared var user: User
        
        var authState: AuthState = .NotLogged
        var login: LoginFeature.State = .init()
        var home: HomeFeature.State
    }
    
    @ObservableState
    enum AuthState: Equatable {
        case Logged
        case NotLogged
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
                case .LoggedIn(let user):
                    state.home.user = user
                    state.authState = .Logged
                default:
                    return .none
                }
            case .homeFeature(let action):
                switch action {
                case .Logout:
                    state.authState = .NotLogged
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
        case .Logged:
            HomeView(store: store.scope(state: \.home, action: \.homeFeature))
        case .NotLogged:
            LoginView(store: store.scope(state: \.login, action: \.loginFeature))
        }
    }
}
