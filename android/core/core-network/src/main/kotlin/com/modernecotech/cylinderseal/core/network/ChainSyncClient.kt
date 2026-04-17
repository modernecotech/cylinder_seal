package com.modernecotech.cylinderseal.core.network

import android.content.Context
import com.modernecotech.cylinderseal.core.datastore.UserPreferences
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import io.grpc.ManagedChannel
import io.grpc.okhttp.OkHttpChannelBuilder
import iq.cbi.cylinderseal.chainsync.ChainSyncGrpcKt
import iq.cbi.cylinderseal.chainsync.CurrencyRateBundle
import iq.cbi.cylinderseal.chainsync.CurrencyRateRequest
import iq.cbi.cylinderseal.chainsync.JournalEntry
import iq.cbi.cylinderseal.chainsync.SyncAck
import java.util.concurrent.TimeUnit
import javax.inject.Singleton
import kotlinx.coroutines.flow.Flow
import kotlinx.coroutines.flow.first
import kotlinx.coroutines.flow.flow

/**
 * gRPC client that streams `JournalEntry`s to a super-peer and receives
 * `SyncAck`s over the bidirectional `SyncChain` RPC.
 *
 * The transport uses OkHttp under the hood so it plays nicely with
 * certificate pinning and Conscrypt for TLS 1.3 support on older Android.
 */
class ChainSyncClient(
    private val channel: ManagedChannel,
) {
    private val stub = ChainSyncGrpcKt.ChainSyncCoroutineStub(channel)

    /**
     * Upload pending entries and emit acks as they arrive. The caller
     * feeds entries by collecting from [outgoing] once the flow is started;
     * the returned flow of acks terminates when the server closes the
     * stream.
     */
    fun syncChain(outgoing: Flow<JournalEntry>): Flow<SyncAck> = stub.syncChain(outgoing)

    suspend fun getCurrencyRates(
        primary: String,
        secondaries: List<String>,
    ): CurrencyRateBundle {
        val req = CurrencyRateRequest.newBuilder()
            .setPrimaryCurrency(primary)
            .addAllSecondaryCurrencies(secondaries)
            .build()
        return stub.getCurrencyRates(req)
    }

    fun shutdown() {
        channel.shutdown().awaitTermination(5, TimeUnit.SECONDS)
    }
}

@Module
@InstallIn(SingletonComponent::class)
object NetworkModule {

    @Provides @Singleton
    fun provideManagedChannel(
        @ApplicationContext ctx: Context,
        prefs: UserPreferences,
    ): ManagedChannel {
        val host = kotlinx.coroutines.runBlocking { prefs.superpeerHost.first() }
        val port = kotlinx.coroutines.runBlocking { prefs.superpeerPort.first() }
        return OkHttpChannelBuilder.forAddress(host, port)
            .useTransportSecurity()
            .keepAliveTime(30, TimeUnit.SECONDS)
            .keepAliveTimeout(10, TimeUnit.SECONDS)
            .build()
    }

    @Provides @Singleton
    fun provideChainSyncClient(channel: ManagedChannel): ChainSyncClient = ChainSyncClient(channel)
}

/** Turn a snapshot list into a cold flow for bidirectional streaming. */
fun snapshotAsFlow(entries: List<JournalEntry>): Flow<JournalEntry> =
    flow { entries.forEach { emit(it) } }
