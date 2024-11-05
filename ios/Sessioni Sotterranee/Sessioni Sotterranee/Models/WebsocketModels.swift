// This file was generated from JSON Schema using quicktype, do not modify it directly.
// To parse the JSON, add this file to your project and do:
//
//   let coordinate = try Coordinate(json)

//
// Hashable or Equatable:
// The compiler will not be able to synthesize the implementation of Hashable or Equatable
// for types that require the use of JSONAny, nor will the implementation of Hashable be
// synthesized for types that have collections (such as arrays or dictionaries).

import Foundation

// MARK: - Coordinate
public struct Coordinate: Codable, Equatable, Sendable {
    public let identifyClient: IdentifyClient?
    public let sentMainChatMessage: SendMainChatMessage?
    public let publishedMainChatMessage: MainChatMessage?
    public let requestMainChatHistory: RequestMainChatHistory?
    public let publishedMainChatHistory: PublishedMainchatHistory?

    public enum CodingKeys: String, CodingKey {
        case identifyClient = "IdentifyClient"
        case sentMainChatMessage = "SentMainChatMessage"
        case publishedMainChatMessage = "PublishedMainChatMessage"
        case requestMainChatHistory = "RequestMainChatHistory"
        case publishedMainChatHistory = "PublishedMainChatHistory"
    }

    public init(identifyClient: IdentifyClient?, sentMainChatMessage: SendMainChatMessage?, publishedMainChatMessage: MainChatMessage?, requestMainChatHistory: RequestMainChatHistory?, publishedMainChatHistory: PublishedMainchatHistory?) {
        self.identifyClient = identifyClient
        self.sentMainChatMessage = sentMainChatMessage
        self.publishedMainChatMessage = publishedMainChatMessage
        self.requestMainChatHistory = requestMainChatHistory
        self.publishedMainChatHistory = publishedMainChatHistory
    }
}

// MARK: Coordinate convenience initializers and mutators

public extension Coordinate {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(Coordinate.self, from: data)
    }

    init(_ json: String, using encoding: String.Encoding = .utf8) throws {
        guard let data = json.data(using: encoding) else {
            throw NSError(domain: "JSONDecoding", code: 0, userInfo: nil)
        }
        try self.init(data: data)
    }

    init(fromURL url: URL) throws {
        try self.init(data: try Data(contentsOf: url))
    }

    func with(
        identifyClient: IdentifyClient?? = nil,
        sentMainChatMessage: SendMainChatMessage?? = nil,
        publishedMainChatMessage: MainChatMessage?? = nil,
        requestMainChatHistory: RequestMainChatHistory?? = nil,
        publishedMainChatHistory: PublishedMainchatHistory?? = nil
    ) -> Coordinate {
        return Coordinate(
            identifyClient: identifyClient ?? self.identifyClient,
            sentMainChatMessage: sentMainChatMessage ?? self.sentMainChatMessage,
            publishedMainChatMessage: publishedMainChatMessage ?? self.publishedMainChatMessage,
            requestMainChatHistory: requestMainChatHistory ?? self.requestMainChatHistory,
            publishedMainChatHistory: publishedMainChatHistory ?? self.publishedMainChatHistory
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

//
// Hashable or Equatable:
// The compiler will not be able to synthesize the implementation of Hashable or Equatable
// for types that require the use of JSONAny, nor will the implementation of Hashable be
// synthesized for types that have collections (such as arrays or dictionaries).

// MARK: - IdentifyClient
public struct IdentifyClient: Codable, Equatable, Sendable {
    public let userID: Int

    public enum CodingKeys: String, CodingKey {
        case userID = "user_id"
    }

    public init(userID: Int) {
        self.userID = userID
    }
}

// MARK: IdentifyClient convenience initializers and mutators

public extension IdentifyClient {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(IdentifyClient.self, from: data)
    }

    init(_ json: String, using encoding: String.Encoding = .utf8) throws {
        guard let data = json.data(using: encoding) else {
            throw NSError(domain: "JSONDecoding", code: 0, userInfo: nil)
        }
        try self.init(data: data)
    }

    init(fromURL url: URL) throws {
        try self.init(data: try Data(contentsOf: url))
    }

    func with(
        userID: Int? = nil
    ) -> IdentifyClient {
        return IdentifyClient(
            userID: userID ?? self.userID
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

//
// Hashable or Equatable:
// The compiler will not be able to synthesize the implementation of Hashable or Equatable
// for types that require the use of JSONAny, nor will the implementation of Hashable be
// synthesized for types that have collections (such as arrays or dictionaries).

// MARK: - PublishedMainchatHistory
public struct PublishedMainchatHistory: Codable, Equatable, Sendable {
    public let messages: [MainChatMessage]
    public let userID: Int

    public enum CodingKeys: String, CodingKey {
        case messages = "messages"
        case userID = "user_id"
    }

    public init(messages: [MainChatMessage], userID: Int) {
        self.messages = messages
        self.userID = userID
    }
}

// MARK: PublishedMainchatHistory convenience initializers and mutators

public extension PublishedMainchatHistory {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(PublishedMainchatHistory.self, from: data)
    }

    init(_ json: String, using encoding: String.Encoding = .utf8) throws {
        guard let data = json.data(using: encoding) else {
            throw NSError(domain: "JSONDecoding", code: 0, userInfo: nil)
        }
        try self.init(data: data)
    }

    init(fromURL url: URL) throws {
        try self.init(data: try Data(contentsOf: url))
    }

    func with(
        messages: [MainChatMessage]? = nil,
        userID: Int? = nil
    ) -> PublishedMainchatHistory {
        return PublishedMainchatHistory(
            messages: messages ?? self.messages,
            userID: userID ?? self.userID
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

//
// Hashable or Equatable:
// The compiler will not be able to synthesize the implementation of Hashable or Equatable
// for types that require the use of JSONAny, nor will the implementation of Hashable be
// synthesized for types that have collections (such as arrays or dictionaries).

// MARK: - MainChatMessage
public struct MainChatMessage: Codable, Equatable, Sendable {
    public let date: Date
    public let messageText: String
    public let sender: MessageSenderUsername

    public enum CodingKeys: String, CodingKey {
        case date = "date"
        case messageText = "message_text"
        case sender = "sender"
    }

    public init(date: Date, messageText: String, sender: MessageSenderUsername) {
        self.date = date
        self.messageText = messageText
        self.sender = sender
    }
}

// MARK: MainChatMessage convenience initializers and mutators

public extension MainChatMessage {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(MainChatMessage.self, from: data)
    }

    init(_ json: String, using encoding: String.Encoding = .utf8) throws {
        guard let data = json.data(using: encoding) else {
            throw NSError(domain: "JSONDecoding", code: 0, userInfo: nil)
        }
        try self.init(data: data)
    }

    init(fromURL url: URL) throws {
        try self.init(data: try Data(contentsOf: url))
    }

    func with(
        date: Date? = nil,
        messageText: String? = nil,
        sender: MessageSenderUsername? = nil
    ) -> MainChatMessage {
        return MainChatMessage(
            date: date ?? self.date,
            messageText: messageText ?? self.messageText,
            sender: sender ?? self.sender
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

public enum MessageSenderUsername: Codable, Equatable, Sendable {
    case enumeration(MessageSenderUsernameEnum)
    case messageSenderUsernameClass(MessageSenderUsernameClass)

    public init(from decoder: Decoder) throws {
        let container = try decoder.singleValueContainer()
        if let x = try? container.decode(MessageSenderUsernameEnum.self) {
            self = .enumeration(x)
            return
        }
        if let x = try? container.decode(MessageSenderUsernameClass.self) {
            self = .messageSenderUsernameClass(x)
            return
        }
        throw DecodingError.typeMismatch(MessageSenderUsername.self, DecodingError.Context(codingPath: decoder.codingPath, debugDescription: "Wrong type for MessageSenderUsername"))
    }

    public func encode(to encoder: Encoder) throws {
        var container = encoder.singleValueContainer()
        switch self {
        case .enumeration(let x):
            try container.encode(x)
        case .messageSenderUsernameClass(let x):
            try container.encode(x)
        }
    }
}

//
// Hashable or Equatable:
// The compiler will not be able to synthesize the implementation of Hashable or Equatable
// for types that require the use of JSONAny, nor will the implementation of Hashable be
// synthesized for types that have collections (such as arrays or dictionaries).

// MARK: - MessageSenderUsernameClass
public struct MessageSenderUsernameClass: Codable, Equatable, Sendable {
    public let other: String

    public enum CodingKeys: String, CodingKey {
        case other = "Other"
    }

    public init(other: String) {
        self.other = other
    }
}

// MARK: MessageSenderUsernameClass convenience initializers and mutators

public extension MessageSenderUsernameClass {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(MessageSenderUsernameClass.self, from: data)
    }

    init(_ json: String, using encoding: String.Encoding = .utf8) throws {
        guard let data = json.data(using: encoding) else {
            throw NSError(domain: "JSONDecoding", code: 0, userInfo: nil)
        }
        try self.init(data: data)
    }

    init(fromURL url: URL) throws {
        try self.init(data: try Data(contentsOf: url))
    }

    func with(
        other: String? = nil
    ) -> MessageSenderUsernameClass {
        return MessageSenderUsernameClass(
            other: other ?? self.other
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

public enum MessageSenderUsernameEnum: String, Codable, Equatable, Sendable {
    case me = "Me"
}

//
// Hashable or Equatable:
// The compiler will not be able to synthesize the implementation of Hashable or Equatable
// for types that require the use of JSONAny, nor will the implementation of Hashable be
// synthesized for types that have collections (such as arrays or dictionaries).

// MARK: - RequestMainChatHistory
public struct RequestMainChatHistory: Codable, Equatable, Sendable {
    public let userID: Int

    public enum CodingKeys: String, CodingKey {
        case userID = "user_id"
    }

    public init(userID: Int) {
        self.userID = userID
    }
}

// MARK: RequestMainChatHistory convenience initializers and mutators

public extension RequestMainChatHistory {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(RequestMainChatHistory.self, from: data)
    }

    init(_ json: String, using encoding: String.Encoding = .utf8) throws {
        guard let data = json.data(using: encoding) else {
            throw NSError(domain: "JSONDecoding", code: 0, userInfo: nil)
        }
        try self.init(data: data)
    }

    init(fromURL url: URL) throws {
        try self.init(data: try Data(contentsOf: url))
    }

    func with(
        userID: Int? = nil
    ) -> RequestMainChatHistory {
        return RequestMainChatHistory(
            userID: userID ?? self.userID
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

//
// Hashable or Equatable:
// The compiler will not be able to synthesize the implementation of Hashable or Equatable
// for types that require the use of JSONAny, nor will the implementation of Hashable be
// synthesized for types that have collections (such as arrays or dictionaries).

// MARK: - SendMainChatMessage
public struct SendMainChatMessage: Codable, Equatable, Sendable {
    public let date: Date
    public let messageText: String
    public let senderID: Int

    public enum CodingKeys: String, CodingKey {
        case date = "date"
        case messageText = "message_text"
        case senderID = "sender_id"
    }

    public init(date: Date, messageText: String, senderID: Int) {
        self.date = date
        self.messageText = messageText
        self.senderID = senderID
    }
}

// MARK: SendMainChatMessage convenience initializers and mutators

public extension SendMainChatMessage {
    init(data: Data) throws {
        self = try newJSONDecoder().decode(SendMainChatMessage.self, from: data)
    }

    init(_ json: String, using encoding: String.Encoding = .utf8) throws {
        guard let data = json.data(using: encoding) else {
            throw NSError(domain: "JSONDecoding", code: 0, userInfo: nil)
        }
        try self.init(data: data)
    }

    init(fromURL url: URL) throws {
        try self.init(data: try Data(contentsOf: url))
    }

    func with(
        date: Date? = nil,
        messageText: String? = nil,
        senderID: Int? = nil
    ) -> SendMainChatMessage {
        return SendMainChatMessage(
            date: date ?? self.date,
            messageText: messageText ?? self.messageText,
            senderID: senderID ?? self.senderID
        )
    }

    func jsonData() throws -> Data {
        return try newJSONEncoder().encode(self)
    }

    func jsonString(encoding: String.Encoding = .utf8) throws -> String? {
        return String(data: try self.jsonData(), encoding: encoding)
    }
}

// MARK: - Helper functions for creating encoders and decoders

func newJSONDecoder() -> JSONDecoder {
    let decoder = JSONDecoder()
    if #available(iOS 10.0, OSX 10.12, tvOS 10.0, watchOS 3.0, *) {
        decoder.dateDecodingStrategy = .iso8601
    }
    return decoder
}

func newJSONEncoder() -> JSONEncoder {
    let encoder = JSONEncoder()
    if #available(iOS 10.0, OSX 10.12, tvOS 10.0, watchOS 3.0, *) {
        encoder.dateEncodingStrategy = .iso8601
    }
    return encoder
}
