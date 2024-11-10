//
//  AppView.swift
//  iosApp
//
//  Created by Christian Belloni on 09/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//


import SwiftUI
import Shared
import KMPObservableViewModelSwiftUI

struct AppView: View {
    @ObservedViewModel var viewModel: AppViewModel
    
    var body: some View {
        HomeView(viewModel: viewModel.homeViewModel)
    }
}
