package com.modernecotech.cylinderseal.feature.business

import kotlinx.serialization.Serializable
import retrofit2.http.Body
import retrofit2.http.DELETE
import retrofit2.http.GET
import retrofit2.http.POST
import retrofit2.http.Path

@Serializable
data class RegisterRequest(
    val user_id: String,
    val account_type: String, // "business_pos" | "business_electronic"
    val legal_name: String,
    val commercial_registration_id: String,
    val tax_id: String,
    val industry_code: String,
    val registered_address: String,
    val contact_email: String,
    val authorized_signer_public_keys_hex: List<String> = emptyList(),
)

@Serializable
data class RegisterResponse(val status: String, val user_id: String)

@Serializable
data class BusinessProfileDto(
    val user_id: String,
    val legal_name: String,
    val commercial_registration_id: String,
    val tax_id: String,
    val industry_code: String,
    val registered_address: String,
    val contact_email: String,
    val signature_threshold: Int,
    val daily_volume_limit_owc: Long,
    val edd_cleared: Boolean,
    val approved_at: String? = null,
)

@Serializable
data class IssueKeyRequest(val label: String, val scopes: List<String> = emptyList())

@Serializable
data class IssueKeyResponse(
    val id: Long,
    val key_prefix: String,
    val secret: String, // Returned exactly once.
    val label: String,
)

@Serializable
data class ApiKeyListItem(
    val id: Long,
    val key_prefix: String,
    val label: String,
    val created_at: String,
    val last_used_at: String? = null,
    val revoked: Boolean,
)

/**
 * Retrofit surface for the `/v1/businesses` endpoints exposed by the
 * super-peer's REST API.
 */
interface BusinessApi {
    @POST("v1/businesses")
    suspend fun register(@Body req: RegisterRequest): RegisterResponse

    @GET("v1/businesses/{userId}")
    suspend fun getProfile(@Path("userId") userId: String): BusinessProfileDto

    @POST("v1/businesses/{userId}/api-keys")
    suspend fun issueApiKey(
        @Path("userId") userId: String,
        @Body req: IssueKeyRequest,
    ): IssueKeyResponse

    @GET("v1/businesses/{userId}/api-keys")
    suspend fun listApiKeys(@Path("userId") userId: String): List<ApiKeyListItem>

    @DELETE("v1/businesses/{userId}/api-keys/{keyId}")
    suspend fun revokeApiKey(
        @Path("userId") userId: String,
        @Path("keyId") keyId: Long,
    )
}
