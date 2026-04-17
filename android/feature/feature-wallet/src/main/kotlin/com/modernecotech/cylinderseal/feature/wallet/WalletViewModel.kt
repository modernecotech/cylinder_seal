package com.modernecotech.cylinderseal.feature.wallet

import androidx.lifecycle.ViewModel
import androidx.lifecycle.viewModelScope
import com.modernecotech.cylinderseal.core.database.PendingEntryDao
import com.modernecotech.cylinderseal.core.database.TransactionDao
import com.modernecotech.cylinderseal.core.database.TransactionEntity
import com.modernecotech.cylinderseal.core.datastore.UserPreferences
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.combine
import kotlinx.coroutines.flow.stateIn

data class WalletUiState(
    val displayName: String = "",
    val balanceMicroOwc: Long = 0,
    val pendingCount: Int = 0,
    val recent: List<TransactionEntity> = emptyList(),
)

@HiltViewModel
class WalletViewModel @Inject constructor(
    private val transactions: TransactionDao,
    private val pending: PendingEntryDao,
    private val prefs: UserPreferences,
) : ViewModel() {

    val uiState: StateFlow<WalletUiState> = combine(
        prefs.displayName,
        transactions.observeBalance(),
        pending.observePendingCount(),
        transactions.observeRecent(),
    ) { name, balance, pendingCount, recent ->
        WalletUiState(
            displayName = name.orEmpty(),
            balanceMicroOwc = balance,
            pendingCount = pendingCount,
            recent = recent,
        )
    }.stateIn(viewModelScope, SharingStarted.Eagerly, WalletUiState())
}
