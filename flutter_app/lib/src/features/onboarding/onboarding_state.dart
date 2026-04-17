import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/platform/hardware_seed.dart';
import '../../core/storage/secure_store.dart';
import '../../rust/api/crypto.dart';
import '../wallet/wallet_state.dart';

/// Coarse derived flag the router uses to decide whether to show the
/// onboarding flow. Recomputed whenever `userPreferencesProvider` is
/// invalidated. Defaults to `false` until the prefs future resolves so
/// the user is always sent to onboarding on cold start with no data.
final isOnboardedProvider = Provider<bool>((ref) {
  final prefs = ref.watch(userPreferencesProvider);
  return prefs.maybeWhen(
    data: (p) => p.isOnboarded,
    orElse: () => false,
  );
});

class OnboardingForm {
  OnboardingForm({
    this.displayName = '',
    this.phoneNumber,
    this.accountType = 'INDIVIDUAL',
    this.businessName,
    this.businessTaxId,
  });

  final String displayName;
  final String? phoneNumber;
  final String accountType;
  final String? businessName;
  final String? businessTaxId;

  OnboardingForm copyWith({
    String? displayName,
    String? phoneNumber,
    String? accountType,
    String? businessName,
    String? businessTaxId,
  }) =>
      OnboardingForm(
        displayName: displayName ?? this.displayName,
        phoneNumber: phoneNumber ?? this.phoneNumber,
        accountType: accountType ?? this.accountType,
        businessName: businessName ?? this.businessName,
        businessTaxId: businessTaxId ?? this.businessTaxId,
      );
}

class OnboardingController extends Notifier<OnboardingForm> {
  @override
  OnboardingForm build() => OnboardingForm();

  void setDisplayName(String v) => state = state.copyWith(displayName: v);
  void setPhone(String v) => state = state.copyWith(phoneNumber: v);
  void setAccountType(String v) => state = state.copyWith(accountType: v);
  void setBusinessName(String v) => state = state.copyWith(businessName: v);
  void setBusinessTaxId(String v) => state = state.copyWith(businessTaxId: v);

  /// Generate keypair, persist hardware seed, and mark onboarding done.
  Future<void> submit() async {
    final store = ref.read(secureStoreProvider);
    final prefs = await ref.read(userPreferencesProvider.future);

    final keypair = await generateKeypair();
    await store.writePrivateKey(keypair.privateKey);
    await store.writePublicKey(keypair.publicKey);

    final seed = await HardwareSeed.read();
    await store.writeHardwareSeed(seed);

    final userId = await userIdFromPublicKey(publicKey: keypair.publicKey);

    await prefs.completeOnboarding(
      displayName: state.displayName.trim(),
      phoneNumber: state.phoneNumber?.trim(),
      accountType: state.accountType,
      userId: userId,
    );

    ref.invalidate(userPreferencesProvider);
    await ref.read(walletProvider.notifier).hydrate();
  }
}

final onboardingControllerProvider =
    NotifierProvider<OnboardingController, OnboardingForm>(
        OnboardingController.new);
