package it.sessionisotterranee.sessionisotterranee

import com.rickclephas.kmp.nativecoroutines.NativeCoroutinesState
import com.rickclephas.kmp.observableviewmodel.ViewModel
import com.rickclephas.kmp.observableviewmodel.MutableStateFlow
import com.rickclephas.kmp.observableviewmodel.launch
import it.sessionisotterranee.sessionisotterranee.api.client.model.User
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow

@Suppress("UNUSED")
class LoginViewModel(private val apiClient: ApiService, private val _user: MutableStateFlow<User?>): ViewModel() {

    private val _newUsername = MutableStateFlow(viewModelScope, "")
    private val _requiresUsername = MutableStateFlow(viewModelScope, false)
    private val _isLoading = MutableStateFlow(viewModelScope, false)
    open class Action {
        object StartLogin: Action()
        object Loading: Action()
        object RequiresUsername: Action()
        data class UsernameChanged(val newUsername: String): Action()
        object SetUsername: Action()
        object LoggedIn: Action()
    }

    @NativeCoroutinesState
    val user = _user.asStateFlow()
    @NativeCoroutinesState
    val newUsername = _newUsername.asStateFlow()
    @NativeCoroutinesState
    val requiresUsername = _requiresUsername.asStateFlow()
    @NativeCoroutinesState
    val isLoading = _isLoading.asStateFlow()

    init {
        viewModelScope.launch {
            _user.value = apiClient.getUser()?.body()
        }
    }

    fun send(action: Action) {
        when(action) {
            is Action.StartLogin -> {
                viewModelScope.launch {
                    apiClient.signIn()
                    apiClient.getUser()?.body()?.let {user ->
                        if (user.username.isNotEmpty()) {
                            send(Action.LoggedIn)
                        } else {
                            send(Action.RequiresUsername)
                        }
                    }
                }
            }
            is Action.Loading -> {
                _isLoading.value = true
            }
            is Action.LoggedIn -> {
                viewModelScope.launch {
                    _user.value = apiClient.getUser()?.body()
                }
            }
            is Action.SetUsername -> {
                viewModelScope.launch {
                    apiClient.setUsername(newUsername.value)
                    send(Action.LoggedIn)
                }
            }
            is Action.RequiresUsername -> {
                _requiresUsername.value = true
            }
            is Action.UsernameChanged -> {
                _newUsername.value = action.newUsername
            }
        }
    }
}