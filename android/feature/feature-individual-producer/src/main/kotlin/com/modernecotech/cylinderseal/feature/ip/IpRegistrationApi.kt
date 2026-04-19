package com.modernecotech.cylinderseal.feature.ip

import kotlinx.serialization.Serializable

/**
 * Request/response DTOs for the citizen-facing IP endpoints on cs-api:
 *   POST /v1/ip/register
 *   GET  /v1/ip/me
 *   GET  /v1/ip/me/rollups
 */
@Serializable
data class IpRegisterRequest(
    val user_id: String,
    val category: String,
    val governorate: String,
    val district: String? = null,
    val display_name: String,
    val attestation_text: String,
)

@Serializable
data class IpRegisterResponse(
    val ip_id: String,
    val category: String,
    val monthly_cap_iqd: Long,
    val status: String,
    val ddpb_badge_ref: String,
)

/**
 * Minimal HTTP surface the ViewModel calls. The concrete implementation is
 * provided by `core-network`'s Ktor client (not included here to keep this
 * feature self-contained; wire-up happens in a DI module).
 */
interface IpRegistrationApi {
    suspend fun register(req: IpRegisterRequest): Result<IpRegisterResponse>
}
