//
//  HomePageView.swift
//  iosApp
//
//  Created by Christian Belloni on 10/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//

import SwiftUI

struct HomePageView<Content: View>: View {
    let title: String
    @ViewBuilder
    var content: () -> Content
    var body: some View {
        VStack(alignment: .leading) {
            PageTitleView(title: title)
                .padding(.horizontal, 20)
            content()
                .padding(.horizontal, 35)
        }
    }
}

#Preview {
    HomePageView(title: "Titolo") {
        
    }
}
