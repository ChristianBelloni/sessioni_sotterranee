//
//  PageTitle.swift
//  iosApp
//
//  Created by Christian Belloni on 10/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//

import SwiftUI

struct PageTitleView: View {
    let title: String
    var body: some View {
        Text(title)
            .font(.title)
        Spacer()
            .frame(height: 25)
    }
}

#Preview {
    PageTitleView(title: "Titolo")
}
