package it.sessionisotterranee.sessionisotterranee

import io.ktor.client.HttpClient
import it.sessionisotterranee.sessionisotterranee.api.client.api.DefaultApi
import it.sessionisotterranee.sessionisotterranee.api.client.invoker.infrastructure.HttpResponse
import it.sessionisotterranee.sessionisotterranee.api.client.model.Event
import it.sessionisotterranee.sessionisotterranee.api.client.model.User



class ApiService(private val authClient: LogtoClientImpl): LogtoClientImpl {
    private val client: DefaultApi = DefaultApi(baseUrl = "http://localhost:8080")
    suspend fun getUser(): HttpResponse<User>? {
        return authClient.bearerToken()?.let {
            return try {
                client.apiUsersMeGet(it)
            } catch(e: Exception) {
                null
            }
        }
    }

    suspend fun getEvents(limit: Long, offset: Long): List<Event> {
        return try {
            client.apiEventsGet(limit, offset).body()
        } catch(e: Exception) {
            listOf()
        }
    }

    suspend fun setUsername(username: String) {
        authClient.bearerToken()?.let { token ->
            try {
                client.apiUsersSetUsernamePatch(token, username)
            } catch(e: Exception) {
                null
            }
        }
    }

    override suspend fun signIn() = this.authClient.signIn()

    override suspend fun signOut() = this.authClient.signOut()

    override suspend fun user() = this.authClient.user()

    override suspend fun bearerToken() = this.authClient.bearerToken()
}