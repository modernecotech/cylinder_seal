package com.modernecotech.cylinderseal.feature.history

import androidx.compose.foundation.layout.Arrangement
import androidx.compose.foundation.layout.Column
import androidx.compose.foundation.layout.fillMaxSize
import androidx.compose.foundation.layout.fillMaxWidth
import androidx.compose.foundation.layout.padding
import androidx.compose.foundation.lazy.LazyColumn
import androidx.compose.foundation.lazy.items
import androidx.compose.material3.Card
import androidx.compose.material3.ExperimentalMaterial3Api
import androidx.compose.material3.MaterialTheme
import androidx.compose.material3.Scaffold
import androidx.compose.material3.Text
import androidx.compose.material3.TopAppBar
import androidx.compose.runtime.Composable
import androidx.compose.runtime.getValue
import androidx.compose.runtime.remember
import androidx.compose.ui.Modifier
import androidx.compose.ui.unit.dp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.ViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewModelScope
import com.modernecotech.cylinderseal.core.common.MoneyFormat
import com.modernecotech.cylinderseal.core.database.TransactionDao
import com.modernecotech.cylinderseal.core.database.TransactionEntity
import dagger.hilt.android.lifecycle.HiltViewModel
import java.text.SimpleDateFormat
import java.util.Date
import java.util.Locale
import javax.inject.Inject
import kotlinx.coroutines.flow.SharingStarted
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.flow.stateIn

@HiltViewModel
class HistoryViewModel @Inject constructor(
    dao: TransactionDao,
) : ViewModel() {
    val transactions: StateFlow<List<TransactionEntity>> =
        dao.observeRecent(limit = 500).stateIn(viewModelScope, SharingStarted.Eagerly, emptyList())
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun HistoryRoute(viewModel: HistoryViewModel = hiltViewModel()) {
    val list by viewModel.transactions.collectAsStateWithLifecycle()
    val dateFmt = remember { SimpleDateFormat("yyyy-MM-dd HH:mm", Locale.getDefault()) }
    Scaffold(topBar = { TopAppBar(title = { Text("History") }) }) { padding ->
        LazyColumn(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(8.dp),
        ) {
            items(list, key = { it.transactionId }) { tx ->
                Card(Modifier.fillMaxWidth()) {
                    Column(Modifier.padding(12.dp)) {
                        Text(
                            (if (tx.direction == "INCOMING") "+" else "-") +
                                MoneyFormat.format(tx.amountMicroOwc, tx.currency),
                            style = MaterialTheme.typography.titleMedium,
                        )
                        Text(
                            tx.counterpartyName ?: tx.counterpartyPublicKeyHex.take(16),
                            style = MaterialTheme.typography.bodyMedium,
                        )
                        Text(
                            "${tx.channel} · ${tx.syncStatus} · " +
                                dateFmt.format(Date(tx.timestampUtc / 1000)),
                            style = MaterialTheme.typography.bodySmall,
                        )
                        if (tx.memo.isNotBlank()) {
                            Text(tx.memo, style = MaterialTheme.typography.bodySmall)
                        }
                    }
                }
            }
        }
    }
}
