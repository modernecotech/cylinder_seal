package com.modernecotech.cylinderseal.feature.ip

import androidx.compose.foundation.background
import androidx.compose.foundation.border
import androidx.compose.foundation.clickable
import androidx.compose.foundation.layout.*
import androidx.compose.foundation.lazy.grid.GridCells
import androidx.compose.foundation.lazy.grid.LazyVerticalGrid
import androidx.compose.foundation.lazy.grid.items
import androidx.compose.foundation.shape.RoundedCornerShape
import androidx.compose.material3.*
import androidx.compose.runtime.*
import androidx.compose.ui.Alignment
import androidx.compose.ui.Modifier
import androidx.compose.ui.draw.clip
import androidx.compose.ui.graphics.Color
import androidx.compose.ui.text.font.FontWeight
import androidx.compose.ui.text.style.TextAlign
import androidx.compose.ui.unit.dp
import androidx.compose.ui.unit.sp
import androidx.hilt.navigation.compose.hiltViewModel
import androidx.lifecycle.ViewModel
import androidx.lifecycle.compose.collectAsStateWithLifecycle
import androidx.lifecycle.viewModelScope
import dagger.hilt.android.lifecycle.HiltViewModel
import javax.inject.Inject
import kotlinx.coroutines.flow.MutableStateFlow
import kotlinx.coroutines.flow.StateFlow
import kotlinx.coroutines.launch

// ---------------------------------------------------------------------------
// State
// ---------------------------------------------------------------------------

data class IpRegistrationState(
    val step: Step = Step.PickCategory,
    val category: IpCategory? = null,
    val displayName: String = "",
    val governorate: String = "",
    val district: String = "",
    val attestationChecked: Boolean = false,
    val submitting: Boolean = false,
    val error: String? = null,
    val registeredBadgeRef: String? = null,
    val monthlyCapIqd: Long = 0L,
) {
    enum class Step { PickCategory, Details, Attestation, Done }
}

@HiltViewModel
class IpRegistrationViewModel @Inject constructor(
    private val api: IpRegistrationApi,
) : ViewModel() {
    private val _state = MutableStateFlow(IpRegistrationState())
    val state: StateFlow<IpRegistrationState> = _state

    fun pickCategory(c: IpCategory) {
        _state.value = _state.value.copy(category = c, step = IpRegistrationState.Step.Details)
    }

    fun setDisplayName(v: String) { _state.value = _state.value.copy(displayName = v) }
    fun setGovernorate(v: String) { _state.value = _state.value.copy(governorate = v) }
    fun setDistrict(v: String) { _state.value = _state.value.copy(district = v) }
    fun setAttestation(v: Boolean) { _state.value = _state.value.copy(attestationChecked = v) }

    fun goToAttestation() {
        val s = _state.value
        if (s.displayName.isBlank() || s.governorate.isBlank()) {
            _state.value = s.copy(error = "Display name and governorate are required")
            return
        }
        _state.value = s.copy(step = IpRegistrationState.Step.Attestation, error = null)
    }

    fun backToDetails() {
        _state.value = _state.value.copy(step = IpRegistrationState.Step.Details)
    }

    /**
     * Submits the registration. `userId` is the caller's Digital Dinar user UUID,
     * injected from the session-holding layer above.
     */
    fun submit(userId: String) {
        val s = _state.value
        val cat = s.category ?: return
        if (!s.attestationChecked) {
            _state.value = s.copy(error = "Attestation required")
            return
        }
        _state.value = s.copy(submitting = true, error = null)
        viewModelScope.launch {
            val req = IpRegisterRequest(
                user_id = userId,
                category = cat.wireName,
                governorate = s.governorate,
                district = s.district.ifBlank { null },
                display_name = s.displayName,
                attestation_text = ATTESTATION_TEXT_AR,
            )
            api.register(req).fold(
                onSuccess = { resp ->
                    _state.value = _state.value.copy(
                        submitting = false,
                        step = IpRegistrationState.Step.Done,
                        registeredBadgeRef = resp.ddpb_badge_ref,
                        monthlyCapIqd = resp.monthly_cap_iqd,
                    )
                },
                onFailure = { t ->
                    _state.value = _state.value.copy(
                        submitting = false,
                        error = t.message ?: "Registration failed",
                    )
                },
            )
        }
    }

    companion object {
        const val ATTESTATION_TEXT_AR: String =
            "أُقرُّ بأن المعلومات أعلاه صحيحة وبأنني سأعمل ضمن السقف الشهري المحدد " +
                "وأنني أوافق على الاحتفاظ بسجل رقمي لنشاطي الإنتاجي."
    }
}

// ---------------------------------------------------------------------------
// UI
// ---------------------------------------------------------------------------

@OptIn(ExperimentalMaterial3Api::class)
@Composable
fun IpRegistrationRoute(
    userId: String,
    onDone: () -> Unit,
    viewModel: IpRegistrationViewModel = hiltViewModel(),
) {
    val state by viewModel.state.collectAsStateWithLifecycle()
    Scaffold(topBar = { TopAppBar(title = { Text(topBarTitle(state.step)) }) }) { padding ->
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(padding)
                .padding(16.dp),
            verticalArrangement = Arrangement.spacedBy(16.dp),
        ) {
            when (state.step) {
                IpRegistrationState.Step.PickCategory -> CategoryGrid(onPick = viewModel::pickCategory)
                IpRegistrationState.Step.Details -> DetailsForm(
                    state = state,
                    onDisplayNameChange = viewModel::setDisplayName,
                    onGovernorateChange = viewModel::setGovernorate,
                    onDistrictChange = viewModel::setDistrict,
                    onNext = viewModel::goToAttestation,
                )
                IpRegistrationState.Step.Attestation -> AttestationForm(
                    state = state,
                    onAttestationChange = viewModel::setAttestation,
                    onBack = viewModel::backToDetails,
                    onSubmit = { viewModel.submit(userId) },
                )
                IpRegistrationState.Step.Done -> DdpbBadge(
                    badgeRef = state.registeredBadgeRef ?: "",
                    category = state.category,
                    monthlyCapIqd = state.monthlyCapIqd,
                    onContinue = onDone,
                )
            }
            state.error?.let { msg ->
                Text(msg, color = MaterialTheme.colorScheme.error)
            }
        }
    }
}

private fun topBarTitle(step: IpRegistrationState.Step): String = when (step) {
    IpRegistrationState.Step.PickCategory -> "اختر نشاطك · Pick your trade"
    IpRegistrationState.Step.Details -> "تفاصيل · Details"
    IpRegistrationState.Step.Attestation -> "تعهد · Attestation"
    IpRegistrationState.Step.Done -> "شارة المنتج · Producer Badge"
}

@Composable
private fun CategoryGrid(onPick: (IpCategory) -> Unit) {
    Text(
        "اختر الفئة التي تمثل نشاطك (60 ثانية)",
        style = MaterialTheme.typography.bodyMedium,
    )
    LazyVerticalGrid(
        columns = GridCells.Fixed(2),
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        verticalArrangement = Arrangement.spacedBy(12.dp),
    ) {
        items(IpCategory.values().toList()) { cat ->
            CategoryCard(cat = cat, onClick = { onPick(cat) })
        }
    }
}

@Composable
private fun CategoryCard(cat: IpCategory, onClick: () -> Unit) {
    Card(
        modifier = Modifier
            .fillMaxWidth()
            .heightIn(min = 112.dp)
            .clickable { onClick() },
    ) {
        Column(
            modifier = Modifier
                .fillMaxSize()
                .padding(12.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.Center,
        ) {
            Text(cat.iconEmoji, fontSize = 32.sp)
            Spacer(Modifier.height(6.dp))
            Text(cat.displayAr, fontWeight = FontWeight.SemiBold)
            Text(cat.displayEn, style = MaterialTheme.typography.bodySmall)
        }
    }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun DetailsForm(
    state: IpRegistrationState,
    onDisplayNameChange: (String) -> Unit,
    onGovernorateChange: (String) -> Unit,
    onDistrictChange: (String) -> Unit,
    onNext: () -> Unit,
) {
    Text(
        state.category?.let { "${it.iconEmoji} ${it.displayAr}" } ?: "",
        style = MaterialTheme.typography.titleMedium,
    )
    OutlinedTextField(
        value = state.displayName,
        onValueChange = onDisplayNameChange,
        label = { Text("اسم العمل / Trading name") },
        modifier = Modifier.fillMaxWidth(),
        singleLine = true,
    )
    OutlinedTextField(
        value = state.governorate,
        onValueChange = onGovernorateChange,
        label = { Text("المحافظة / Governorate") },
        modifier = Modifier.fillMaxWidth(),
        singleLine = true,
    )
    OutlinedTextField(
        value = state.district,
        onValueChange = onDistrictChange,
        label = { Text("الناحية / District (optional)") },
        modifier = Modifier.fillMaxWidth(),
        singleLine = true,
    )
    Button(
        onClick = onNext,
        modifier = Modifier.fillMaxWidth(),
    ) { Text("التالي · Next") }
}

@OptIn(ExperimentalMaterial3Api::class)
@Composable
private fun AttestationForm(
    state: IpRegistrationState,
    onAttestationChange: (Boolean) -> Unit,
    onBack: () -> Unit,
    onSubmit: () -> Unit,
) {
    Text("تعهّد المنتج · Producer attestation", style = MaterialTheme.typography.titleMedium)
    Text(
        IpRegistrationViewModel.ATTESTATION_TEXT_AR,
        style = MaterialTheme.typography.bodyMedium,
    )
    Text(
        "Monthly cap: 7,000,000 IQD. Exceeding it triggers graduation to a " +
            "formal SME producer. A presumptive micro-tax of 1.0–1.5% is " +
            "withheld from receipts; 60% goes to your social-security pot.",
        style = MaterialTheme.typography.bodySmall,
        color = MaterialTheme.colorScheme.onSurfaceVariant,
    )
    Row(verticalAlignment = Alignment.CenterVertically) {
        Checkbox(
            checked = state.attestationChecked,
            onCheckedChange = onAttestationChange,
        )
        Spacer(Modifier.width(4.dp))
        Text("أوافق · I agree", modifier = Modifier.padding(start = 4.dp))
    }
    Row(
        horizontalArrangement = Arrangement.spacedBy(12.dp),
        modifier = Modifier.fillMaxWidth(),
    ) {
        OutlinedButton(
            onClick = onBack,
            modifier = Modifier.weight(1f),
            enabled = !state.submitting,
        ) { Text("رجوع · Back") }
        Button(
            onClick = onSubmit,
            modifier = Modifier.weight(1f),
            enabled = state.attestationChecked && !state.submitting,
        ) {
            if (state.submitting) {
                CircularProgressIndicator(
                    modifier = Modifier.size(18.dp),
                    strokeWidth = 2.dp,
                )
            } else {
                Text("تسجيل · Register")
            }
        }
    }
}

@Composable
private fun DdpbBadge(
    badgeRef: String,
    category: IpCategory?,
    monthlyCapIqd: Long,
    onContinue: () -> Unit,
) {
    Card(
        modifier = Modifier.fillMaxWidth(),
        colors = CardDefaults.cardColors(containerColor = MaterialTheme.colorScheme.primaryContainer),
    ) {
        Column(
            modifier = Modifier
                .fillMaxWidth()
                .padding(24.dp),
            horizontalAlignment = Alignment.CenterHorizontally,
            verticalArrangement = Arrangement.spacedBy(12.dp),
        ) {
            Text("DDPB", style = MaterialTheme.typography.displaySmall, fontWeight = FontWeight.Bold)
            Text(
                "شارة المنتج الرقمي المحلي",
                style = MaterialTheme.typography.titleMedium,
                textAlign = TextAlign.Center,
            )
            Text(
                "Digital Domestic Producer Badge",
                style = MaterialTheme.typography.bodySmall,
                textAlign = TextAlign.Center,
            )
            category?.let {
                Text(
                    "${it.iconEmoji} ${it.displayAr} · ${it.displayEn}",
                    style = MaterialTheme.typography.titleMedium,
                )
            }
            Box(
                modifier = Modifier
                    .fillMaxWidth()
                    .background(Color.White, RoundedCornerShape(8.dp))
                    .border(1.dp, Color.Black.copy(alpha = 0.2f), RoundedCornerShape(8.dp))
                    .padding(12.dp),
            ) {
                Text(
                    badgeRef,
                    style = MaterialTheme.typography.bodySmall,
                    textAlign = TextAlign.Center,
                    modifier = Modifier.fillMaxWidth(),
                )
            }
            Text(
                "Monthly cap: %,d IQD".format(monthlyCapIqd),
                style = MaterialTheme.typography.bodyMedium,
            )
        }
    }
    Button(onClick = onContinue, modifier = Modifier.fillMaxWidth()) { Text("متابعة · Continue") }
}
