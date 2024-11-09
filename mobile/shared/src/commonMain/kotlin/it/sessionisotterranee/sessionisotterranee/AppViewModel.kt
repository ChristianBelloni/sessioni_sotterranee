package it.sessionisotterranee.sessionisotterranee

import com.rickclephas.kmp.observableviewmodel.ViewModel
import com.rickclephas.kmp.observableviewmodel.launch

data class AppViewModel(val loginViewModel: LoginViewModel): ViewModel() {
}