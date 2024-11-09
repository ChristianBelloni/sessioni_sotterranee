package it.sessionisotterranee.sessionisotterranee

interface Platform {
    val name: String
}

expect fun getPlatform(): Platform


@Suppress("UNUSED")
interface LogtoClientImpl {
    suspend fun signIn()
    suspend fun signOut()
    suspend fun user(): LogtoUser
    suspend fun bearerToken(): String?
}

data class LogtoUser(val sub: String, val username: String?)