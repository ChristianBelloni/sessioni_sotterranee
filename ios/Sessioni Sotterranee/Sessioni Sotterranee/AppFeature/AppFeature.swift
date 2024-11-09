//
//  AppFeature.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 05/11/24.
//

import Foundation
import SwiftUI
import ComposableArchitecture

@Reducer
struct AppFeature {
    @Dependency(\.apiClient) var apiClient
    
    @ObservableState
    struct State: Equatable {
        @Shared var user: User?
        var mainChat: MainChatFeature.State
        var home: HomeFeature.State
    }
    
    enum Action {
        case mainChat(MainChatFeature.Action)
        case home(HomeFeature.Action)
    }
    
    var body: some ReducerOf<Self> {
        Scope(state: \.home, action: \.home) {
            HomeFeature()
        }
        
        Scope(state: \.mainChat, action: \.mainChat) {
            MainChatFeature()
        }
        
        Reduce { state, action in
            return .none
        }
    }
    
    static func initialState(user: Shared<User?>) -> State {
        return State(user: user, mainChat: MainChatFeature.State(user: user), home: .init(featuredEvents: .init(), mainChat: .init(user: user)))
    }
}


struct AppView : View {
    let store: StoreOf<AppFeature>
    var body: some View {
        HomeView(store: store.scope(state: \.home, action: \.home))
        // MainChatView(store: store.scope(state: \.mainChat, action: \.mainChat))
    }
}
