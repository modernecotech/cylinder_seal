import 'dart:typed_data';

import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/storage/secure_store.dart';
import '../../rust/api/crypto.dart';

class Wallet {
  Wallet({
    required this.userId,
    required this.publicKey,
    required this.displayName,
    required this.accountType,
  });

  final String userId;
  final Uint8List publicKey;
  final String displayName;
  final String accountType;
}

class WalletNotifier extends AsyncNotifier<Wallet?> {
  @override
  Future<Wallet?> build() async {
    final prefs = await ref.watch(userPreferencesProvider.future);
    final store = ref.watch(secureStoreProvider);
    if (!prefs.isOnboarded) return null;
    final pk = await store.readPublicKey();
    if (pk == null) return null;
    final uid = prefs.userId ?? await userIdFromPublicKey(publicKey: pk);
    return Wallet(
      userId: uid,
      publicKey: pk,
      displayName: prefs.displayName ?? 'User',
      accountType: prefs.accountType,
    );
  }

  /// Used by the onboarding flow once a keypair has been generated and
  /// the display-name + account-type captured.
  Future<void> hydrate() async {
    state = const AsyncLoading();
    state = AsyncData(await build());
  }
}

final walletProvider =
    AsyncNotifierProvider<WalletNotifier, Wallet?>(WalletNotifier.new);

/// Token balance shown on the wallet home — wired to a stub that would
/// hit the gRPC sync endpoint in production. For now it returns 0 so the
/// UI renders without a live backend.
final balanceProvider = FutureProvider<int>((ref) async {
  final wallet = await ref.watch(walletProvider.future);
  if (wallet == null) return 0;
  return 0;
});
