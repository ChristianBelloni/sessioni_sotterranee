package it.sessionisotterranee.sessionisotterranee

import com.rickclephas.kmp.nativecoroutines.NativeCoroutinesState
import com.rickclephas.kmp.observableviewmodel.ViewModel
import kotlinx.coroutines.flow.Flow
import com.rickclephas.kmp.observableviewmodel.MutableStateFlow
import com.rickclephas.kmp.observableviewmodel.launch
import kotlinx.coroutines.flow.asStateFlow

class LoginViewModel(val authClient: LogtoClientImpl): ViewModel() {
    private val _user = MutableStateFlow<LogtoUser?>(viewModelScope, null)

    @NativeCoroutinesState
    val user = _user.asStateFlow()

    init {
        viewModelScope.launch {
            _user.value = authClient.user()
        }
    }

    suspend fun signIn() {
        authClient.signIn()
        this._user.value = authClient.user()
    }
}