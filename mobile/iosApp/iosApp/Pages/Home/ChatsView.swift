//
//  ChatsView.swift
//  iosApp
//
//  Created by Christian Belloni on 10/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//

import SwiftUI

struct ChatsView: View {
    var body: some View {
        HomePageView(title: "Discussioni") {
            ChatPreview(
                title: "Info",
                sender: "Carlo",
                lastMessage: "lorem ipsum dolor sit amet consectetur adipiscing elite",
                date: .now
            )
            Divider()
            ChatPreview(
                title: "Compra/Vendita strumenti",
                sender: "Franco",
                lastMessage: "lorem ipsum dolor sit amet consectetur adipiscing elite",
                date: .now
            )
            Divider()
            ChatPreview(
                title: "Cerca musicisti",
                sender: "Carlo",
                lastMessage: "lorem ipsum dolor sit amet consectetur adipiscing elite",
                date: .now
            )
            Divider()
            ChatPreview(
                title: "Suggerisci evento",
                sender: "Franco",
                lastMessage: "lorem ipsum dolor sit amet consectetur adipiscing elite",
                date: .now
            )
            Divider()
            Spacer()
        }
    }
}

struct ChatPreview: View {
    let title: String
    let sender: String
    let lastMessage: String
    let date: Date
    
    var body: some View {
        Button {
            
        } label: {
            VStack(alignment: .leading) {
                Text(title)
                HStack {
                    HStack {
                        Text("\(sender): ").font(.footnote)
                        Text(lastMessage)
                            .lineLimit(1)
                            .truncationMode(.tail)
                            .font(.footnote)
                    }.padding(3)
                    Spacer()
                    Text(date.eventDate()).font(.footnote)
                }
            }
        }.foregroundStyle(.black)
    }
}

#Preview {
    ChatsView()
}
