package com.modernecotech.cylinderseal.core.database

import androidx.room.Dao
import androidx.room.Insert
import androidx.room.OnConflictStrategy
import androidx.room.Query
import kotlinx.coroutines.flow.Flow

@Dao
interface TransactionDao {
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: TransactionEntity)

    @Query("SELECT * FROM transactions ORDER BY timestamp_utc DESC LIMIT :limit")
    fun observeRecent(limit: Int = 50): Flow<List<TransactionEntity>>

    @Query("SELECT * FROM transactions WHERE transactionId = :id")
    suspend fun get(id: String): TransactionEntity?

    @Query("""
        UPDATE transactions SET sync_status = :status
        WHERE transactionId = :id
    """)
    suspend fun updateStatus(id: String, status: String)

    @Query("""
        SELECT COALESCE(SUM(CASE
            WHEN direction = 'INCOMING' THEN amount_micro_owc
            ELSE -amount_micro_owc END), 0)
        FROM transactions WHERE sync_status IN ('CONFIRMED','PENDING')
    """)
    fun observeBalance(): Flow<Long>
}

@Dao
interface PendingEntryDao {
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(entity: PendingEntryEntity)

    @Query("SELECT * FROM pending_entries ORDER BY sequence_number ASC")
    suspend fun drain(): List<PendingEntryEntity>

    @Query("DELETE FROM pending_entries WHERE entryHashHex = :hash")
    suspend fun delete(hash: String)

    @Query("""
        UPDATE pending_entries SET attempt_count = attempt_count + 1,
                                   last_attempt_at = :now
        WHERE entryHashHex = :hash
    """)
    suspend fun recordAttempt(hash: String, now: Long)

    @Query("SELECT COUNT(*) FROM pending_entries")
    fun observePendingCount(): Flow<Int>

    @Query("SELECT MAX(sequence_number) FROM pending_entries")
    suspend fun latestSequence(): Long?
}

@Dao
interface ContactDao {
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun upsert(entity: ContactEntity)

    @Query("SELECT * FROM contacts ORDER BY last_seen_at DESC")
    fun observeAll(): Flow<List<ContactEntity>>

    @Query("SELECT * FROM contacts WHERE publicKeyHex = :pk")
    suspend fun get(pk: String): ContactEntity?
}

@Dao
interface NonceChainDao {
    @Insert(onConflict = OnConflictStrategy.REPLACE)
    suspend fun insert(entity: NonceChainEntity)

    @Query("SELECT * FROM nonce_chain ORDER BY counter DESC LIMIT 1")
    suspend fun latest(): NonceChainEntity?

    @Query("SELECT counter FROM nonce_chain ORDER BY counter DESC LIMIT 1")
    suspend fun latestCounter(): Long?
}
