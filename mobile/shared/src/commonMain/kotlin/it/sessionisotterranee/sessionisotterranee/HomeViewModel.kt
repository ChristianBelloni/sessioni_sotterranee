package it.sessionisotterranee.sessionisotterranee

import com.rickclephas.kmp.nativecoroutines.NativeCoroutinesState
import com.rickclephas.kmp.observableviewmodel.MutableStateFlow
import com.rickclephas.kmp.observableviewmodel.ViewModel
import com.rickclephas.kmp.observableviewmodel.launch
import it.sessionisotterranee.sessionisotterranee.api.client.model.Event
import it.sessionisotterranee.sessionisotterranee.api.client.model.User
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.asStateFlow
import kotlinx.coroutines.flow.update

@Suppress("UNUSED")
class HomeViewModel(private val apiClient: ApiService, user: MutableStateFlow<User?>): ViewModel() {
    @NativeCoroutinesState
    val user = user.asStateFlow()

    private val _events =  MutableStateFlow(viewModelScope, mutableListOf<Event>())
    @NativeCoroutinesState
    val events = _events.asStateFlow()

    private val _isLoading = MutableStateFlow(viewModelScope, false)
    @NativeCoroutinesState
    val isLoading = _isLoading.asStateFlow()

    private var currentIndex = MutableStateFlow(viewModelScope, 0L)
    private val batchSize = 7L
    open class Action {
        data object Load: Action()
        data object Refresh: Action()
        data class Loaded(val events: List<Event>): Action()
    }

    fun send(action: Action) {
        when(action) {
            is Action.Load -> {
                viewModelScope.launch {
                    val newEvents = apiClient.getEvents(
                        limit = batchSize,
                        offset = currentIndex.value
                    )
                    currentIndex.value += batchSize
                    send(Action.Loaded(newEvents))
                }
            }
            is Action.Refresh -> {
                _isLoading.value = true
                currentIndex.value = 0L
                _events.value.clear()
                send(Action.Load)
            }
            is Action.Loaded -> {
                _events.update {
                    it.addAll(action.events)
                    it
                }
                _isLoading.value = false
            }
        }
    }
}