//
//  WebSocketClient.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 05/11/24.
//

import Foundation
import ComposableArchitecture

protocol WebSocketClientProtocol {
    func connect() async
    func identifyClient(userId: Int) async throws
    func receiveMessage() async throws -> String?
    func sendMessage(user: User, message: String) async throws
    var isConnected: Bool { get }
}

class WebSocketClient: NSObject, WebSocketClientProtocol, URLSessionWebSocketDelegate {
    var isConnected: Bool = false
    
    func sendMessage(user: User, message: String) async throws {
        let msg = Coordinate(identifyClient: nil, sentMainChatMessage: SendMainChatMessage(date: Date.now, messageText: message, senderID: user.id), publishedMainChatMessage: nil, requestMainChatHistory: nil, publishedMainChatHistory: nil)
        try await task.send(.string(try msg.jsonString()!))
    }
    
    let task: URLSessionWebSocketTask
    
    init(url: URL) {
        task = URLSession.shared.webSocketTask(with: url)
        
        super.init()
        task.delegate = self
    }
    
    func connect() async {
        task.resume()
        await withCheckedContinuation { continuation in
            self.onConnect = { _ in
                continuation.resume()
            }
        }
    }
    
    func identifyClient(userId: Int) async throws {
        let message = Coordinate.init(identifyClient: IdentifyClient(userID: userId), sentMainChatMessage: nil, publishedMainChatMessage: nil, requestMainChatHistory: nil, publishedMainChatHistory: nil)
        try await task.send(.string(try message.jsonString()!))
    }
    
    func urlSession(_ session: URLSession, webSocketTask: URLSessionWebSocketTask, didOpenWithProtocol protocol: String?) {
        self.onConnect(nil)
        self.isConnected = true
    }
    
    var onConnect: ((any Error)?) -> Void = { _ in }
    
    func receiveMessage() async throws -> String? {
        
        switch try await task.receive() {
        case .data(let data):
            String(data: data, encoding: .utf8)
        case .string(let data):
            data
        @unknown default:
            fatalError("unexpected message kind")
        }
    }
    
    
}


private enum WebSocketClientKey: DependencyKey{
    static let liveValue: any WebSocketClientProtocol = WebSocketClient(url: URL(string:"ws://localhost:8080/ws")!)
}

extension DependencyValues {
    var webSocketClient: WebSocketClientProtocol {
        get { self[WebSocketClientKey.self] }
        set { self[WebSocketClientKey.self] = newValue }
      }
}
