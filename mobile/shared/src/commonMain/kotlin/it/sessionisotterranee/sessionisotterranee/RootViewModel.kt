package it.sessionisotterranee.sessionisotterranee

import com.rickclephas.kmp.nativecoroutines.NativeCoroutinesState
import com.rickclephas.kmp.observableviewmodel.MutableStateFlow
import com.rickclephas.kmp.observableviewmodel.ViewModel
import com.rickclephas.kmp.observableviewmodel.launch
import it.sessionisotterranee.sessionisotterranee.api.client.model.User
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

class RootViewModel(private val apiClient: ApiService): ViewModel() {
    private val _user = MutableStateFlow<User?>(viewModelScope, null)
    val loginViewModel: LoginViewModel = LoginViewModel(apiClient, _user)
    val appviewModel = AppViewModel(apiClient, _user)

    @NativeCoroutinesState
    val user = _user.asStateFlow()
}

