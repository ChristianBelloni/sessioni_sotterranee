//
//  BottomTabBar.swift
//  iosApp
//
//  Created by Christian Belloni on 10/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//

import SwiftUI

struct TabPage : Identifiable, Hashable{
    let id: CGFloat
    let icon: String
}

struct BottomTabBar: View {
    let pages: [TabPage]
    @Binding var selectedTab: CGFloat
    
    let onClick: () -> Void
    
    var body: some View {
        HStack(alignment: .center) {
            ForEach(pages.enumerated().map{ $0 }, id: \.element.id) { index, page in
                Button {
                    selectedTab = page.id
                    onClick()
                } label: {
                    Image(systemName: page.icon)
                        .bottomBarImage(selected: selectedTab == page.id)
                        .foregroundStyle(.white)
                }
                
                if index != pages.count - 1 {
                    Spacer()
                }
            }
        }
        .padding(.vertical, 10)
        .padding(.horizontal, 10)
        .background(.gray.opacity(0.95))
        .clipShape(RoundedRectangle(cornerRadius: 30))
    }
}

struct ExampleTabPage: View {
    var body: some View {
        Text("Example page")
    }
}

struct ExampleTabPage2: View {
    var body: some View {
        Text("Example page2")
    }
}

struct ExampleTabPage3: View {
    var body: some View {
        Text("Example page2")
    }
}

extension Image {
    @ViewBuilder
    func bottomBarImage(selected: Bool) -> some View {
        let size: CGFloat = 25
        let padding: CGFloat = 10
        if selected {
            self.resizable()
            .scaledToFit()
            .frame(width: size, height: size)
            .padding(padding)
            .background(.secondary.opacity(0.5), in: Circle())
        } else {
            self.resizable()
            .scaledToFit()
            .frame(width: size, height: size)
            .padding(padding)
            
        }
    }
}

@available(iOS 17.0, macOS 11.0, tvOS 14.0, watchOS 7.0, *)
#Preview {
    @Previewable @State var selectedTab: CGFloat = 0.0
    VStack {
        switch selectedTab {
        case 0: ExampleTabPage()
        case 1: ExampleTabPage2()
        case 2: ExampleTabPage3()
        default: EmptyView()
        }
        Spacer()
        BottomTabBar(pages: [
            TabPage(id: 1, icon: "star.fill"),
            TabPage(id: 0, icon: "house.fill"),
            TabPage(id: 2, icon: "map.fill")
        ], selectedTab: $selectedTab) {
            
        }
    }
}
