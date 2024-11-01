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
    
    @ObservableState
    struct State: Equatable {
        var error: String?
    }
    
    enum Action {
        case LaunchLogin
        case LoggedIn
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
                        await send(.LoggedIn)
                    } catch {
                        await send(.Error(error.localizedDescription))
                    }
                }
            case .Error(let error):
                state.error = error
            default:
                return .none
            }
            return .none
        }
    }
}

struct LoginView : View {
    let store: StoreOf<LoginFeature>
    
    var body: some View {
        VStack {
            Text("Sessioni Sotterranee")
            Button("Login") {
                store.send(.LaunchLogin)
            }
        }

    }
}
