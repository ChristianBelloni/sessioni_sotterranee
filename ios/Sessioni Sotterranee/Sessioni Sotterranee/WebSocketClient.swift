//
//  WebSocketClient.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 05/11/24.
//

import Foundation
import ComposableArchitecture
import Combine

protocol WebSocketClientProtocol {
    func connect() async
    
    func registerClient(userId: Int) async throws
    
    func subscribe(onReceive: @MainActor @escaping (WebsocketMessage) -> Void)
    
    func sendMessage(user: User, text: String) async throws
    
    func disconnect()
}

struct WebSocketClient {
    var task: URLSessionWebSocketTask
    var stream: PassthroughSubject<WebsocketMessage, Never>
    private var cancellables: LockIsolated<Set<AnyCancellable>> = .init([])
    var isConnected: LockIsolated<Bool> = .init(false)
    
    init(url: URL) {
        self.task = URLSession.shared.webSocketTask(with: url)
        self.stream = .init()
    }
    
    func openStream() {
        Task {
            while self.task.state == .running {
                let result = try await self.task.receive()
                switch result {
                case .data(let data):
                    if let msg = try? WebsocketMessage(data: data) {
                        stream.send(msg)
                    }
                case .string(let string):
                    if let msg = try? WebsocketMessage(string) {
                        stream.send(msg)
                    }
                @unknown default:
                    return
                }
            }
        }
    }
}

extension WebSocketClient : WebSocketClientProtocol {
    
    class Delegate: NSObject, URLSessionWebSocketDelegate {
        var continuation: CheckedContinuation<Void, Never>
        init(continuation: CheckedContinuation<Void, Never>) {
            self.continuation = continuation
        }
        
        func urlSession(_ session: URLSession, webSocketTask: URLSessionWebSocketTask, didOpenWithProtocol protocol: String?) {
            continuation.resume()
        }
    }
    
    func connect() async {
        if isConnected.value {
            return
        }
        await withCheckedContinuation { continuation in
            let delegate = Delegate(continuation: continuation)
            self.task.delegate = delegate
            self.task.resume()
        }
        isConnected.withValue({ $0 = true })
        self.openStream()
    }
    
    func registerClient(userId: Int) async throws {
        try await self.task.send(.string(try WebsocketMessage.identifyClient(userID: userId).jsonString()!))
    }
    
    func subscribe(onReceive: @MainActor @escaping (WebsocketMessage) -> Void) {
        cancellables.withValue({ inner in
            self.stream.receive(on: DispatchQueue.main).sink { msg in MainActor.assumeIsolated { onReceive(msg) } }.store(in: &inner)
        })
        
    }
    
    func sendMessage(user: User, text: String) async throws {
        try await self.task.send(try .string(WebsocketMessage.sentMainChatMessage(senderID: user.id, messageText: text, date: Date.now).jsonString()!))
    }
    
    func disconnect() {
        task.cancel()
    }
}

fileprivate enum WebSocketClientKey : DependencyKey {
    static let liveValue: any WebSocketClientProtocol = WebSocketClient(url: URL(string: "ws://localhost:8080/ws")!)
}

extension DependencyValues {
    var websocketClient: WebSocketClientProtocol {
        get { self[WebSocketClientKey.self] }
        set { self[WebSocketClientKey.self] = newValue }
    }
}
