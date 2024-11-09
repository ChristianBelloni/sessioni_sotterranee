package it.sessionisotterranee.sessionisotterranee

import io.ktor.client.HttpClient
import it.sessionisotterranee.sessionisotterranee.api.client.api.DefaultApi
import it.sessionisotterranee.sessionisotterranee.api.client.model.User
import org.openapitools.client.infrastructure.HttpResponse


class ApiService(private val client: DefaultApi, private val authClient: LogtoClientImpl) {
    suspend fun getUser(): HttpResponse<User>? {
        return authClient.bearerToken()?.let {
            client.apiUsersMeGet(it)
        }
    }
}