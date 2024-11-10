//
//  HomeView.swift
//  iosApp
//
//  Created by Christian Belloni on 09/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//

import SwiftUI
import KMPObservableViewModelSwiftUI
import Shared

struct HomeView: View {
    @ObservedViewModel var viewModel: HomeViewModel
    @State var selectedTab: CGFloat = 0.0
    init(viewModel: HomeViewModel) {
        self.viewModel = viewModel
    }
    
    let animationDuration = 0.1
    @State var proxy: GeometryProxy?
    @State var currentOffset = 0.0
    
    var body: some View {
        GeometryReader { proxy in
            
            ZStack {
                
                VStack(alignment: .center) {
                    Spacer()
                    Text("Bookmarked")
                    Spacer()
                }
                .contentShape(Rectangle())
                .tabView(proxy: proxy, index: -2, selectedIndex: $selectedTab)
                
                ChatsView()
                .contentShape(Rectangle())
                .tabView(proxy: proxy, index: -1, selectedIndex: $selectedTab)
                
                FeaturedEventsView(viewModel: viewModel)
                    .tabView(proxy: proxy, index: 0, selectedIndex: $selectedTab)
                VStack(alignment: .center) {
                    Spacer()
                    Text("Explore")
                    Spacer()
                }
                .contentShape(Rectangle())
                .tabView(proxy: proxy, index: 1, selectedIndex: $selectedTab)
                
                VStack(alignment: .center) {
                    Spacer()
                    Text("Profile")
                    Spacer()
                }
                .contentShape(Rectangle())
                .tabView(proxy: proxy, index: 2, selectedIndex: $selectedTab)
                
                
                VStack {
                    Spacer()
                    BottomTabBar(pages: pages, selectedTab: $selectedTab) {
                        currentOffset = selectedTab
                    }
                }.padding(.horizontal, 15)
            }
            .padding(.bottom, 10)
            .onAppear() {
                viewModel.send(action: HomeViewModel.ActionLoad())
            }
            .onAppear() {
                self.proxy = proxy
            }
        }.gesture (
            DragGesture()
                .onChanged { value in
                    selectedTab = -(value.translation.width / (proxy?.size.width ?? 1.0)) + currentOffset
                    
                    print("change", selectedTab)
                }.onEnded { _ in
                    selectedTab = min(max(selectedTab.rounded(), -1), 1)
                    print("end", selectedTab)
                    currentOffset = selectedTab
                }
        )
    }
}

extension View {
    @ViewBuilder
    func tabView(proxy: GeometryProxy, index: CGFloat, selectedIndex: Binding<CGFloat>) -> some View {
        TabContent(proxy: proxy, index: index, selectedIndex: selectedIndex) {
            self
        }
    }
}

struct TabContent<Content: View>: View {
    var proxy: GeometryProxy
    var index: CGFloat
    @Binding var selectedIndex: CGFloat
    
    let animationDuration = 0.1
    
    @ViewBuilder
    var content: () -> Content
    
    var body: some View {
        var offset: CGFloat {
            let idx = selectedIndex - index
            return -proxy.size.width * CGFloat(idx)
        }
        
        content()
            .padding(.top, 70)
            .offset(x: offset)
            .animation(.linear(duration: animationDuration), value: offset)
    }
}


let pages: [TabPage] = [
    TabPage(id: -2, icon: "star"),
    TabPage(id: -1, icon: "message"),
    TabPage(id: 0, icon: "house.fill"),
    TabPage(id: 1, icon: "map"),
    TabPage(id: 2, icon: "person")
]
