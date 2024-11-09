//
//  HomeFeature.swift
//  Sessioni Sotterranee
//
//  Created by Christian Belloni on 05/11/24.
//

import Foundation
import ComposableArchitecture
import SwiftUI
import MapKit

@Reducer
struct HomeFeature {
    
    @ObservableState
    struct State: Equatable {
        var featuredEvents: FeaturedEventsFeature.State
        var mainChat: MainChatFeature.State
    }
    
    enum Action {
        case featuredEvents(FeaturedEventsFeature.Action)
        case mainChat(MainChatFeature.Action)
    }
    
    var body: some ReducerOf<Self> {
        Scope(state: \.featuredEvents, action: \.featuredEvents) {
            FeaturedEventsFeature()
        }
        
        Scope(state: \.mainChat, action: \.mainChat) {
            MainChatFeature()
        }
        
        Reduce { state, action in
            return .none
        }
    }
}

struct HomeView : View {
    @Dependency(\.websocketClient) var websocketClient
    @State private var selectedTab: Tab = Tab.home
    
    init(store: StoreOf<HomeFeature>) {
        self.store = store
        UITabBar.appearance().isHidden = true
    }
    
    let store: StoreOf<HomeFeature>
    var body: some View {
        ZStack(alignment: .bottom) {
            TabView(selection: $selectedTab) {
                VStack {
                    
                }.tag(Tab.bookmark)
                MainChatView(store: store.scope(state: \.mainChat, action: \.mainChat)).tag(Tab.explore)
                FeaturedEventsView(store: store.scope(state: \.featuredEvents, action: \.featuredEvents))
                    .onAppear() {
                        store.send(.featuredEvents(.Load))
                    }
                    .tag(Tab.home)
                VStack {
                    
                }.tag(Tab.notification)
                VStack {
                    
                }.tag(Tab.profile)
            }
            .padding(.top, 60)
            CustomBottomTabBarView(currentTab: $selectedTab)
                .padding(.bottom)
        }
    }
}




@Reducer
struct FeaturedEventsFeature {
    @Dependency(\.apiClient) var apiClient
    static let saveURL = try! FileManager.default.url(for: .applicationSupportDirectory, in: .userDomainMask, appropriateFor: nil, create: true).appending(path: "events2.data")
    
    @ObservableState
    struct State : Equatable {
        var events: [Event] = []
        var currentIndex = 0
        let batchSize = 7
        var isLoading = false
        
    }
    
    enum Action {
        case Load
        case Loaded([Event])
        case Refresh
    }
    
    var body: some ReducerOf<Self> {
        Reduce { state, action in
            switch action {
            case .Load:
                // state.isLoading = true
                return .run { [offset = state.currentIndex, limit = state.batchSize] send in
                    let events = try await apiClient.loadEvents(limit: limit, offset: offset)
                    await send(.Loaded(events))
                }
            case .Loaded(let events):
                for event in events {
                    if !state.events.contains(where: { $0.id == event.id }) {
                        state.events.append(event)
                    }
                }
                
                state.currentIndex += state.batchSize
                state.isLoading = false
                return .none
            case .Refresh:
                state.isLoading = true
                state.currentIndex = 0
                state.events.removeAll()
                return .run { send in
                    await send(.Load)
                }
            }
        }
    }
}

struct FeaturedEventsView: View {
    let store: StoreOf<FeaturedEventsFeature>
    var body: some View {
        let withIndex = store.events.enumerated().map({ $0 })
        VStack {
            Text("Prossimi eventi").font(.largeTitle)
            if store.isLoading {
                ProgressView().progressViewStyle(.circular)
            } else {
                List(withIndex, id: \.element.id) { index, item in
                    EventView(event: item).onAppear() {
                        if index == withIndex.count - 2 {
                            store.send(.Load)
                        }
                    }
                }.listStyle(.inset)
                .refreshable {
                    if store.events.isEmpty {
                        store.send(.Refresh)
                    }
                }
            }
        }
    }
}


struct EventView : View {
    let event: Event
    var body: some View {
        VStack {
            AsyncImage(url: URL(string:"https://lh3.googleusercontent.com/p/AF1QipN0r9BDYdpxz6NEZgm4c-xfUyzsZlAeQUNShc8U=s1360-w1360-h1020")) { image in
                image.image?
                    .resizable()
                    .aspectRatio(contentMode: .fit)
            }
            HStack {
                VStack {
                    Text(event.title)
                        .font(.title)
                        .padding(.vertical, 5)
                    
                    Text(event.description ?? "Nessuna descrizione")
                        .lineLimit(5)
                }
            }.padding()
        }
        .clipShape(RoundedRectangle(cornerRadius: 10))
        .overlay(RoundedRectangle(cornerRadius: 10)
            .stroke(Color.gray, lineWidth: 1)
            .shadow(radius: 1))
        .padding([.top, .horizontal])
    }
}


private let buttonDimen: CGFloat = 55

struct CustomBottomTabBarView: View {
    
    @Binding var currentTab: Tab
    
    var body: some View {
        HStack {
             TabBarButton(imageName: Tab.bookmark.rawValue)
                .frame(width: buttonDimen, height: buttonDimen)
                .onTapGesture {
                    currentTab = .bookmark
                }           
           
            Spacer()

            TabBarButton(imageName: Tab.explore.rawValue)
                .frame(width: buttonDimen, height: buttonDimen)
                .onTapGesture {
                    currentTab = .explore
                }

            Spacer()
         
            TabBarButton(imageName: Tab.home.rawValue)
                .frame(width: buttonDimen, height: buttonDimen)
                .onTapGesture {
                    currentTab = .home
                }
            

            Spacer()
            
            TabBarButton(imageName: Tab.notification.rawValue)
                .frame(width: buttonDimen, height: buttonDimen)
                .onTapGesture {
                    currentTab = .notification
                }
            
            Spacer()
            
            TabBarButton(imageName: Tab.profile.rawValue)
                .frame(width: buttonDimen, height: buttonDimen)
                .onTapGesture {
                    currentTab = .profile
                }

        }
        .frame(width: (buttonDimen * CGFloat(Tab.allCases.count)) + 60)
        .tint(Color.black)
        .padding(.vertical, 2.5)
        .background(Color.white)
        .clipShape(Capsule(style: .continuous))
        .overlay {
            SelectedTabCircleView(currentTab: $currentTab)
        }
        .shadow(color: Color.gray.opacity(0.5), radius: 5, x: 0, y: 10)
        .animation(.interactiveSpring(response: 0.5, dampingFraction: 0.65, blendDuration: 0.65), value: currentTab)
    }
    
}

private struct TabBarButton: View {
    let imageName: String
    var body: some View {
        Image(systemName: imageName)
            .renderingMode(.template)
            .tint(.black)
            .fontWeight(.bold)
    }
}

enum Tab: String, Hashable, CaseIterable {
    case explore = "message"
    case bookmark = "star"
    case home = "house"
    case notification = "bell"
    case profile = "person"
}

struct SelectedTabCircleView: View {
    
    @Binding var currentTab: Tab
    
    private var horizontalOffset: CGFloat {
        switch currentTab {
        case .bookmark:
            return -138
        case .explore:
            return -72
        case .home:
            return 0
        case .notification:
            return 72
        case .profile:
            return 138
        }
    }
    
    var body: some View {
        ZStack {
            Circle()
                .fill(Color.blue)
                .frame(width: buttonDimen , height: buttonDimen)
            
            TabBarButton(imageName: "\(currentTab.rawValue).fill")
                .foregroundColor(.white)
        }
        .offset(x: horizontalOffset)
    }

}
