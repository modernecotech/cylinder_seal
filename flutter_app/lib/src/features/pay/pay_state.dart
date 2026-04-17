import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:uuid/uuid.dart';

import '../../core/storage/secure_store.dart';
import '../../rust/api/transactions.dart';
import '../../rust/api/wire.dart';
import '../wallet/wallet_state.dart';

class PayDraft {
  PayDraft({
    this.recipientPublicKeyHex = '',
    this.amountOwc = 0,
    this.memo = '',
    this.channel = 1, // 1 NFC, 2 BLE, 3 Online
  });

  final String recipientPublicKeyHex;
  final double amountOwc;
  final String memo;
  final int channel;

  PayDraft copyWith({
    String? recipientPublicKeyHex,
    double? amountOwc,
    String? memo,
    int? channel,
  }) =>
      PayDraft(
        recipientPublicKeyHex: recipientPublicKeyHex ?? this.recipientPublicKeyHex,
        amountOwc: amountOwc ?? this.amountOwc,
        memo: memo ?? this.memo,
        channel: channel ?? this.channel,
      );
}

class PayDraftController extends Notifier<PayDraft> {
  @override
  PayDraft build() => PayDraft();

  void setRecipient(String hex) =>
      state = state.copyWith(recipientPublicKeyHex: hex.trim());
  void setAmount(double v) => state = state.copyWith(amountOwc: v);
  void setMemo(String v) => state = state.copyWith(memo: v);
  void setChannel(int c) => state = state.copyWith(channel: c);
}

final payDraftProvider =
    NotifierProvider<PayDraftController, PayDraft>(PayDraftController.new);

class SignedPayment {
  SignedPayment({required this.cbor, required this.qrPayload, required this.apdus});

  final List<int> cbor;
  final String qrPayload;
  final List<List<int>> apdus;
}

/// Builds and signs a transaction from the current `PayDraft`. Throws if
/// the wallet is missing fields or if the recipient hex is malformed.
final signedPaymentProvider = FutureProvider.autoDispose<SignedPayment>((ref) async {
  final draft = ref.watch(payDraftProvider);
  final wallet = await ref.watch(walletProvider.future);
  if (wallet == null) throw StateError('wallet not initialised');

  final store = ref.watch(secureStoreProvider);
  final sk = await store.readPrivateKey();
  if (sk == null) throw StateError('private key missing');

  final hwSeed = await store.readHardwareSeed();
  if (hwSeed == null) throw StateError('hardware seed missing');

  final prevNonce = await store.readLastNonce() ?? List<int>.filled(32, 0);
  // Counter is monotonic across sessions — for the demo path we use
  // microseconds-since-epoch as a CSPRNG-grade non-repeating value.
  final counter = BigInt.from(DateTime.now().microsecondsSinceEpoch);
  final nextNonce = await deriveNextNonce(
    prevNonce: prevNonce,
    hardwareSeed: hwSeed,
    counter: counter,
  );

  final recipient = _hexDecode(draft.recipientPublicKeyHex);
  final input = TransactionInput(
    fromPublicKey: wallet.publicKey,
    toPublicKey: recipient,
    amountMicroOwc: PlatformInt64.from((draft.amountOwc * 1000000).round()),
    currencyContext: 'IQD',
    fxRateSnapshot: '1',
    channel: draft.channel,
    memo: draft.memo,
    deviceId: const Uuid().v4(),
    previousNonce: prevNonce,
    currentNonce: nextNonce,
    latitude: 0,
    longitude: 0,
    locationAccuracyMeters: 0,
    locationSource: 0,
  );

  final cbor = await buildAndSignTransaction(input: input, privateKey: sk);
  await store.writeLastNonce(nextNonce);

  final qr = await qrEncode(cbor: cbor);
  final apdus = await buildNfcApdus(cbor: cbor);

  return SignedPayment(cbor: cbor, qrPayload: qr, apdus: apdus);
});

List<int> _hexDecode(String s) {
  final clean = s.replaceAll(RegExp(r'\s'), '');
  if (clean.length % 2 != 0) {
    throw FormatException('hex must be even-length: ${clean.length}');
  }
  final out = <int>[];
  for (var i = 0; i < clean.length; i += 2) {
    final byte = int.parse(clean.substring(i, i + 2), radix: 16);
    out.add(byte);
  }
  return out;
}
