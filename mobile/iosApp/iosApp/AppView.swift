import SwiftUI
import Shared
import KMPObservableViewModelSwiftUI

struct AppView: View {
    @StateViewModel var appViewModel: AppViewModel
    
    var body: some View {
        VStack {
            LoginView(viewModel: appViewModel.loginViewModel)
        }
        .frame(maxWidth: .infinity, maxHeight: .infinity, alignment: .top)
        .padding()
    }
}

