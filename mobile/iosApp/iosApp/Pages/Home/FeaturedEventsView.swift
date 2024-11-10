//
//  FeaturedEventsView.swift
//  iosApp
//
//  Created by Christian Belloni on 10/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//

import SwiftUI
import KMPObservableViewModelSwiftUI
import Shared

struct FeaturedEventsView: View {
    @ObservedViewModel var viewModel: HomeViewModel
    
    var body: some View {
        let withIndex = viewModel.events.map({ $0 as! Event }).enumerated().map({ $0 })
        HomePageView(title: "Prossimi eventi") {
            ScrollView(showsIndicators: false) {
                LazyVStack {
                    ForEach(withIndex, id: \.element.id) { index, item in
                        EventView(event: item).onAppear() {
                            if index == withIndex.count - 1 {
                                viewModel.send(action: HomeViewModel.ActionLoad())
                            }
                        }
                    }
                }
            }.refreshable {
                viewModel.send(action: HomeViewModel.ActionRefresh())
            }
        }
    }
}


struct EventView : View {
    let event: Event
    @State var opacity: Double = 1.0
    
    var body: some View {
        VStack {
            AsyncImage(url: URL(string:"https://lh3.googleusercontent.com/p/AF1QipN0r9BDYdpxz6NEZgm4c-xfUyzsZlAeQUNShc8U=s1360-w1360-h1020")) { image in
                image.image?
                    .resizable()
                    .aspectRatio(contentMode: .fit)
            }
            .overlay {
                VStack {
                    if #available(iOS 16.0, *) {
                        HStack(alignment: .center) {
                            Text(event.title.capitalized)
                                .font(.title)
                                .padding(.vertical, 5)
                            Spacer()
                            Text(Date(timeIntervalSince1970: Double(event.date.epochSeconds)).eventDate())
                        }
                        .padding(5)
                        .padding(.horizontal, 10)
                        .background(Color.white.opacity(0.9))
                        .clipShape(UnevenRoundedRectangle(cornerRadii: RectangleCornerRadii(bottomTrailing: 10 )))
                    } else {
                        HStack(alignment: .center) {
                            Text(event.title)
                                .font(.title)
                                .padding(.vertical, 5)
                            Spacer()
                            Text(Date(timeIntervalSince1970: Double(event.date.epochSeconds)).eventDate())
                        }
                        .padding()
                        .background(Color.white)
                    }
                    Spacer()
                }
                .padding(.trailing, 50)
                
            }
            HStack {
                VStack {
                    Text(event.description_ ?? "Nessuna descrizione")
                        .lineLimit(5)
                }
            }.padding()
        }
        .clipShape(RoundedRectangle(cornerRadius: 10))
        .overlay(RoundedRectangle(cornerRadius: 10)
            .stroke(Color.gray, lineWidth: 1)
            .shadow(radius: 1))
        .padding([.top])
        .opacity(opacity)
        .simultaneousGesture(DragGesture(minimumDistance: 0).onChanged({ _ in
            opacity = 0.9
        }).onEnded({ _ in
            opacity = 1
        }))
        
    }
}

extension Date {
    func eventDate() -> String {
        let formatter = DateFormatter()
        formatter.dateFormat = "dd MMM"
        
        let date = formatter.string(from: self)
        return "\(date) \(self.formatted(date: .omitted, time: .shortened))"
    }
}
