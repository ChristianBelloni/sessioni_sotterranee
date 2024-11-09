//
//  LoginView.swift
//  iosApp
//
//  Created by Christian Belloni on 09/11/24.
//  Copyright © 2024 orgName. All rights reserved.
//
import SwiftUI
import Shared
import KMPObservableViewModelSwiftUI

struct LoginView: View {
    @StateViewModel var viewModel: LoginViewModel
    @State var user: LogtoUser? = nil
    
    var body: some View {
        if viewModel.user != nil {
            VStack {
                Text("Hello, \(viewModel.user!.username ?? "undefined")!")
            }
        }
        Button("Login") {
            Task {
                try await viewModel.signIn()
            }
        }
    }
}
