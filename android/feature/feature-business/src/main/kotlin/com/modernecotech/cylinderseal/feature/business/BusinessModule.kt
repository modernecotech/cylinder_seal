package com.modernecotech.cylinderseal.feature.business

import com.modernecotech.cylinderseal.core.datastore.UserPreferences
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.runBlocking
import kotlinx.serialization.json.Json
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import retrofit2.Retrofit
import retrofit2.converter.kotlinx.serialization.asConverterFactory

@Module
@InstallIn(SingletonComponent::class)
object BusinessModule {
    @Provides @Singleton
    fun provideBusinessApi(prefs: UserPreferences): BusinessApi {
        val host = runBlocking { prefs.superpeerHost.first() }
        // REST always runs on HTTPS 443 in production; dev override via
        // BuildConfig can lower it. Keeping the convention simple here.
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
            .create(BusinessApi::class.java)
    }
}
