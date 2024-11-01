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
    struct State: Equatable { }
    enum Action {
        case LaunchLogout
        case Logout
    }
    
    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .LaunchLogout:
                return .run { send in
                    await authClient.signOut()
                    await send(.Logout)
                }
            default:
                return .none
            }
        }
    }
}


struct HomeView: View {
    let store: StoreOf<HomeFeature>
    
    var body: some View {
        VStack {
            Text("Hello")
            Button("Logout") {
                store.send(.LaunchLogout)
            }
        }
    }
}
