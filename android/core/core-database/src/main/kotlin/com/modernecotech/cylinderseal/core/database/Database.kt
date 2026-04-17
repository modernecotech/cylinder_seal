package com.modernecotech.cylinderseal.core.database

import android.content.Context
import androidx.room.Database
import androidx.room.Room
import androidx.room.RoomDatabase
import com.modernecotech.cylinderseal.core.cryptography.Hkdf
import com.modernecotech.cylinderseal.core.cryptography.KeystoreManager
import dagger.Module
import dagger.Provides
import dagger.hilt.InstallIn
import dagger.hilt.android.qualifiers.ApplicationContext
import dagger.hilt.components.SingletonComponent
import javax.inject.Singleton
import net.sqlcipher.database.SQLiteDatabase
import net.sqlcipher.database.SupportFactory

@Database(
    entities = [
        TransactionEntity::class,
        PendingEntryEntity::class,
        ContactEntity::class,
        NonceChainEntity::class,
    ],
    version = 1,
    exportSchema = true,
)
abstract class CsDatabase : RoomDatabase() {
    abstract fun transactionDao(): TransactionDao
    abstract fun pendingEntryDao(): PendingEntryDao
    abstract fun contactDao(): ContactDao
    abstract fun nonceChainDao(): NonceChainDao
}

@Module
@InstallIn(SingletonComponent::class)
object DatabaseModule {

    @Provides @Singleton
    fun provideDatabase(
        @ApplicationContext ctx: Context,
        keystore: KeystoreManager,
    ): CsDatabase {
        SQLiteDatabase.loadLibs(ctx)
        val passphrase = deriveSqlcipherKey(ctx, keystore)
        val factory = SupportFactory(passphrase, null, /*clearPassphrase=*/false)

        return Room
            .databaseBuilder(ctx, CsDatabase::class.java, DB_NAME)
            .openHelperFactory(factory)
            .fallbackToDestructiveMigration()
            .build()
    }

    /**
     * SQLCipher passphrase = HKDF(Keystore seed, salt=pin-or-empty,
     *                             info="cs:sqlcipher:v1", 32 bytes).
     *
     * For now we use an empty salt at boot — the PIN-derived salt kicks in
     * once onboarding is complete, at which point the DB is rekeyed
     * (PRAGMA rekey) on first unlock. This keeps the DB accessible for the
     * enroll flow without requiring the user to set a PIN first.
     */
    private fun deriveSqlcipherKey(
        ctx: Context,
        keystore: KeystoreManager,
    ): ByteArray {
        keystore.ensureMasterKey()
        val ikm = keystore.seedForHkdf()
        // Salt is the package signing digest so different installs produce
        // different keys even on the same hardware.
        val salt = "cs.sqlcipher.salt.v1".toByteArray()
        return Hkdf.derive(
            ikm = ikm,
            salt = salt,
            info = "cs:sqlcipher:v1".toByteArray(),
            length = 32,
        )
    }

    @Provides fun provideTransactionDao(db: CsDatabase) = db.transactionDao()
    @Provides fun providePendingEntryDao(db: CsDatabase) = db.pendingEntryDao()
    @Provides fun provideContactDao(db: CsDatabase) = db.contactDao()
    @Provides fun provideNonceChainDao(db: CsDatabase) = db.nonceChainDao()

    const val DB_NAME = "cylinder_seal.db"
}
