//
//  MainChatFeature.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 05/11/24.
//

import Foundation
import SwiftUI
import ComposableArchitecture

@Reducer
struct MainChatFeature {
    @Dependency(\.webSocketClient) var websocketClient

    @ObservableState
    struct State : Equatable{
        @Shared var user: User
        var newMessage = ""
        var isLoading = true
    }
    
    enum Action {
        case Connected
        case SendMessage
        case NewMessageChanged(String)
        case ReceiveMessage
        case RequestHistory
        case ReceiveHistory
    }
    
    var body: some Reducer<State, Action> {
        Reduce { state, action in
            switch action {
            case .NewMessageChanged(let newMessage):
                state.newMessage = newMessage
                return .none
            case .SendMessage:
                return .run { [message = state.newMessage, user = state.user] send in
                    try await websocketClient.sendMessage(user: user, message: message)
            }
            case .Connected:
                state.isLoading = false
                return .none
            default:
                return .none
            }
        }
    }
}

struct MainChatView : View {
    @Bindable var store: StoreOf<MainChatFeature>
    
    var body: some View {
        if store.isLoading {
            ProgressView().progressViewStyle(.circular)
        } else {
            VStack {
                TextField("...", text: $store.newMessage.sending(\.NewMessageChanged))
                Button("Send") {
                    store.send(.SendMessage)
                }
            }
        }
    }
}
