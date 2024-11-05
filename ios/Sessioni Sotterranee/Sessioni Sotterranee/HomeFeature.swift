//
//  HomeFeature.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 31/10/24.
//

import Foundation
import SwiftUI
import ComposableArchitecture


@Reducer
struct HomeFeature {
    @Dependency(\.authClient) var authClient
    
    @ObservableState
    struct State: Equatable {
        @Shared var user: User
        var navigation: HomeNavigation = .Mainchat
        var mainChat: MainChatFeature.State
    }
    
    @ObservableState
    enum HomeNavigation: Equatable {
        case Mainchat
        case Profile
    }
    
    enum Action {
        case GoToMainChat
        case GoToProfile
        case LaunchLogout
        case Logout
        case mainChat(MainChatFeature.Action)
    }
    
    var body: some Reducer<State, Action> {
        Scope(state: \.mainChat, action: \.mainChat) {
            MainChatFeature()
        }
        
        Reduce { state, action in
            switch action {
            case .LaunchLogout:
                return .run { send in
                    await authClient.signOut()
                    await send(.Logout)
                }
            case .GoToProfile:
                state.navigation = .Profile
                return .none
            case .GoToMainChat:
                state.navigation = .Mainchat
                state.mainChat = MainChatFeature.State(user: state.$user)
                return .none
            default:
                return .none
            }
        }
    }
}


struct HomeView: View {
    @Dependency(\.webSocketClient) var websocketClient
    let store: StoreOf<HomeFeature>
    
    var body: some View {
        Group {
            switch store.state.navigation {
            case .Mainchat:
                MainChatView(store: store.scope(state: \.mainChat, action: \.mainChat))
            case .Profile:
                VStack {
                    Text("Hello")
                    Button("Logout") {
                        store.send(.LaunchLogout)
                    }
                    Button("GotoMainChat") {
                        store.send(.GoToMainChat)
                    }
                }
            }
        }.task() {
            await websocketClient.connect()
            do {
                try await websocketClient.identifyClient(userId: store.state.user.id)
                store.send(.mainChat(.Connected))
            } catch {
                print(error.localizedDescription)
            }
        }
    }
}
