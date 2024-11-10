package it.sessionisotterranee.sessionisotterranee

import com.rickclephas.kmp.observableviewmodel.ViewModel
import it.sessionisotterranee.sessionisotterranee.api.client.model.User
import kotlinx.coroutines.flow.MutableStateFlow


class AppViewModel(private val apiClient: ApiService, private val _user: MutableStateFlow<User?>): ViewModel() {
    val homeViewModel = HomeViewModel(apiClient, _user)
}

