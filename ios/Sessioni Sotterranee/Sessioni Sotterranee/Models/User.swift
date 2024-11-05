//
//  User.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 05/11/24.
//

import Foundation

@Observable
class User : Equatable {
    static func == (lhs: User, rhs: User) -> Bool {
        lhs.id == rhs.id
    }
    
    var id: Int = 0
    var logtoId: String = ""
    var username: String = ""
    
    convenience init(id: Int, logtoId: String, username: String) {
        self.init()
        self.id = id
        self.logtoId = logtoId
        self.username = username
    }
}
