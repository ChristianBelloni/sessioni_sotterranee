//
//  MainChatFeature.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 05/11/24.
//

import Foundation
import ComposableArchitecture
import SwiftUI
import Combine



@Reducer
struct MainChatFeature {
    @Dependency(\.websocketClient) var websocketClient
    var cancellables: LockIsolated<Set<AnyCancellable>> = .init(.init())
    static let saveURL = try! FileManager.default.url(for: .applicationSupportDirectory, in: .userDomainMask, appropriateFor: nil, create: true).appending(path: "mainChat.data")

    @ObservableState
    struct State: Equatable {
        @Shared var user: User?
        @Shared(.fileStorage(MainChatFeature.saveURL)) var messages: [MainChatMessage] = []

        
        var isLoading = true
        var message = ""
    }
    
    enum Action { 
        case Connect
        case RegisterClient
        case Loaded
        case MessageReceived(WebsocketMessage)
        case MessageChanged(String)
        case SendMessage
        case MessageSent
    }
    
    var body: some ReducerOf<Self> {
        Reduce { state, action in
            switch action {
            case .Connect:
                return .run { send in
                    await websocketClient.connect()
                    await send(.RegisterClient)
                }
            case .RegisterClient:
                return .run { [ user = state.user ] send in
                    if let user = user {
                        try await websocketClient.registerClient(userId: user.id)
                        await send(.Loaded)
                    }
                }
            case .Loaded:
                state.isLoading = false
                return .none
            case .MessageReceived(let message):
                if let msg = message.publishedMainChatMessage {
                    state.messages.append(msg)
                }
                return .none
            case .MessageChanged(let updatedMessage):
                state.message = updatedMessage
                return .none
            case .SendMessage:
                return .run { [user = state.user, text = state.message] send in
                    if user != nil {
                        try await websocketClient.sendMessage(user: user!, text: text)
                        await send(.MessageSent)
                    }
                }
            case .MessageSent:
                state.messages.append(MainChatMessage(date: Date.now, messageText: state.message, sender: MessageSenderUsername.enumeration(.me)))
                state.message = ""
                return .none
            }
        }
    }
}


struct MainChatView : View {
    @Bindable var store: StoreOf<MainChatFeature>
    
    var body: some View {
        Group {
            
            if store.isLoading {
                ProgressView().progressViewStyle(.circular).onAppear {
                    store.send(.Connect)
                }
            } else {
                VStack {
                    MessageList(store: store)
                    Spacer()
                    Divider()
                    HStack {
                        TextEditor(text: $store.message.sending(\.MessageChanged)).textEditorStyle(.plain).frame(maxHeight: 50)
                        Button("Send") {
                            store.send(.SendMessage)
                        }
                    }.padding(.horizontal, 10)
                }.padding(20)
                    .padding(.vertical, 30)
            }
        }
    }
}

struct MessageList : View {
    @Bindable var store: StoreOf<MainChatFeature>
    
    var body: some View {
        
        // https://medium.com/@karsonbraaten/easily-group-objects-by-a-date-property-in-swift-e803d450f30e
        var messagesByDay: [(Date,[(Int, MainChatMessage)])] {
            let empty: [Date: [MainChatMessage]] = [:]
            return store.messages.reduce(into: empty) { acc, cur in
                let components = Calendar.current.dateComponents([.year, .month, .day, .hour], from: cur.date)
                let date = Calendar.current.date(from: components)!
                let existing = acc[date] ?? []
                acc[date] = existing + [cur]
            }.map({ (k,v) in (k, v.sorted(by: { a, b in a.date < b.date })
                .enumerated()
                .map{ i, a in (i, a) } ) })
            .sorted(by: { a, b in a.0 < b.0 })
        }
        
        ScrollView {
            ForEach(messagesByDay, id: \.0) { msgs in
                MessageSection(msgs: msgs)
            }
        }
        .defaultScrollAnchor(.bottom)
    }
}

struct MessageSection: View {
    let msgs: (Date, [(Int, MainChatMessage)])
    
    var body: some View {
        Section(msgs.0.formatted(date: Date.FormatStyle.DateStyle.abbreviated, time: Date.FormatStyle.TimeStyle.shortened)) {
            ForEach(msgs.1, id: \.0) { (i, msg) in
                switch msg.sender {
                case .enumeration(_):
                    UserMessage(msg: msg)
                case .messageSenderUsernameClass(let other):
                    OtherMessage(msg: msg, other: other.other)
                }
            }
        }
    }
}

struct UserMessage : View {
    let msg: MainChatMessage
    var body: some View {
        HStack {
            Spacer()
            VStack(alignment: .trailing) {
                Text(msg.messageText)
                Text(msg.date.formatted(date: Date.FormatStyle.DateStyle.omitted, time: Date.FormatStyle.TimeStyle.shortened))
                    .font(.footnote)
            }
            .messageStyle(.accent)
        }
    }
}

struct OtherMessage: View {
    let msg: MainChatMessage
    let other: String
    
    var body: some View {
        HStack {
            VStack(alignment: .trailing) {
                Text(other)
                Text(msg.messageText)
                Text(msg.date.formatted(date: Date.FormatStyle.DateStyle.omitted, time: Date.FormatStyle.TimeStyle.shortened))
                    .font(.footnote)
            }
            .messageStyle(.secondaryAccent)
            Spacer()
        }
    }
}

extension View {
    func messageStyle(_ color: Color) -> some View {
        self.padding(5)
            .padding(.horizontal, 7)
            .background(color)
            .clipShape(.buttonBorder)
    }
}
