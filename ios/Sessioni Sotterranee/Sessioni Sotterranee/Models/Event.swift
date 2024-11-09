//
//  Event.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 05/11/24.
//

import Foundation
import OpenAPIRuntime

struct Event: Codable, Equatable, Identifiable {
    let id: Int
    let title: String
    let date: Date
    let description: String?
    let genre: String?
    let location: String?
    let url: String?
    let image: String?
    
    init(_ data: Components.Schemas.Event) {
        id = Int(data.id)
        title = data.title
        date = data.date
        description = data.description
        genre = data.genre
        location = data.location
        url = data.url
        image = data.image
    }
}
