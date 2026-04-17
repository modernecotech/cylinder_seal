package com.modernecotech.cylinderseal.feature.history

import com.modernecotech.cylinderseal.core.datastore.UserPreferences
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import retrofit2.Retrofit
import retrofit2.converter.kotlinx.serialization.asConverterFactory
import retrofit2.http.GET
import retrofit2.http.Path
import retrofit2.http.Query

/**
 * One row from `/v1/compliance/users/:user_id/explanations`. Serialised
 * by serde on the server with snake_case fields — match exactly.
 */
@Serializable
data class RecentEvalDto(
    val transaction_id: String,
    val composite_score: Int,
    val risk_level: String,
    val held_for_review: Boolean,
    val recommended_action: String,
    val explanation: String,
    val evaluated_at: String,
)

@Serializable
data class UserExplanationResponse(
    val user_id: String,
    val recent: List<RecentEvalDto>,
)

interface ComplianceApi {
    @GET("v1/compliance/users/{userId}/explanations")
    suspend fun explanations(
        @Path("userId") userId: String,
        @Query("limit") limit: Int = 20,
    ): UserExplanationResponse
}

@Module
@InstallIn(SingletonComponent::class)
object ComplianceApiModule {
    @Provides @Singleton
    fun provideComplianceApi(prefs: UserPreferences): ComplianceApi {
        val host = runBlocking { prefs.superpeerHost.first() }
        val baseUrl = "https://$host/"

        val client = OkHttpClient.Builder()
            .addInterceptor(okhttp3.logging.HttpLoggingInterceptor().apply {
                level = okhttp3.logging.HttpLoggingInterceptor.Level.BASIC
            })
            .build()

        val json = Json {
            ignoreUnknownKeys = true
            encodeDefaults = true
        }

        return Retrofit.Builder()
            .baseUrl(baseUrl)
            .client(client)
            .addConverterFactory(json.asConverterFactory("application/json".toMediaType()))
            .build()
            .create(ComplianceApi::class.java)
    }
}
