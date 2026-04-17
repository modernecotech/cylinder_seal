package com.modernecotech.cylinderseal.core.datastore

import android.content.Context
import androidx.datastore.preferences.core.Preferences
import androidx.datastore.preferences.core.booleanPreferencesKey
import androidx.datastore.preferences.core.edit
import androidx.datastore.preferences.core.longPreferencesKey
import androidx.datastore.preferences.core.stringPreferencesKey
import androidx.datastore.preferences.preferencesDataStore
import dagger.hilt.android.qualifiers.ApplicationContext
import javax.inject.Inject
import javax.inject.Singleton
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.map

private val Context.dataStore by preferencesDataStore(name = "cs_prefs")

@Singleton
class UserPreferences @Inject constructor(@ApplicationContext private val ctx: Context) {

    private object Keys {
        val ONBOARDED = booleanPreferencesKey("onboarded")
        val DISPLAY_NAME = stringPreferencesKey("display_name")
        val PHONE_NUMBER = stringPreferencesKey("phone_number")
        val KYC_TIER = stringPreferencesKey("kyc_tier")
        val SUPERPEER_HOST = stringPreferencesKey("superpeer_host")
        val SUPERPEER_PORT = longPreferencesKey("superpeer_port")
        val LAST_SYNC_AT = longPreferencesKey("last_sync_at")
    }

    val isOnboarded: Flow<Boolean> = ctx.dataStore.data.map { it[Keys.ONBOARDED] ?: false }
    val displayName: Flow<String?> = ctx.dataStore.data.map { it[Keys.DISPLAY_NAME] }
    val phoneNumber: Flow<String?> = ctx.dataStore.data.map { it[Keys.PHONE_NUMBER] }
    val kycTier: Flow<String> = ctx.dataStore.data.map { it[Keys.KYC_TIER] ?: "ANONYMOUS" }
    val superpeerHost: Flow<String> =
        ctx.dataStore.data.map { it[Keys.SUPERPEER_HOST] ?: DEFAULT_HOST }
    val superpeerPort: Flow<Int> =
        ctx.dataStore.data.map { (it[Keys.SUPERPEER_PORT] ?: DEFAULT_PORT).toInt() }
    val lastSyncAt: Flow<Long> = ctx.dataStore.data.map { it[Keys.LAST_SYNC_AT] ?: 0L }

    suspend fun completeOnboarding(
        displayName: String,
        phoneNumber: String?,
    ) {
        ctx.dataStore.edit { p ->
            p[Keys.ONBOARDED] = true
            p[Keys.DISPLAY_NAME] = displayName
            phoneNumber?.let { p[Keys.PHONE_NUMBER] = it }
        }
    }

    suspend fun setKycTier(tier: String) {
        ctx.dataStore.edit { it[Keys.KYC_TIER] = tier }
    }

    suspend fun setSuperpeer(host: String, port: Int) {
        ctx.dataStore.edit {
            it[Keys.SUPERPEER_HOST] = host
            it[Keys.SUPERPEER_PORT] = port.toLong()
        }
    }

    suspend fun recordSync(nowMillis: Long) {
        ctx.dataStore.edit { it[Keys.LAST_SYNC_AT] = nowMillis }
    }

    /** Used by tests / settings "forget this device". */
    suspend fun reset() {
        ctx.dataStore.edit(Preferences::clear)
    }

    companion object {
        const val DEFAULT_HOST = "sp-baghdad.cbi.iq"
        const val DEFAULT_PORT = 50051L
    }
}
