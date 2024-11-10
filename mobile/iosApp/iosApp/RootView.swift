import SwiftUI
import Shared
import KMPObservableViewModelSwiftUI

struct RootView: View {
    @StateViewModel var appViewModel: RootViewModel
    
    var body: some View {
        if appViewModel.user != nil {
            AppView(viewModel: appViewModel.appviewModel)
        } else {
            LoginView(viewModel: appViewModel.loginViewModel)
        }
    }
}


