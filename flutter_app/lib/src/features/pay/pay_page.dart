import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_scanner/mobile_scanner.dart';
import 'package:qr_flutter/qr_flutter.dart';

import '../../core/platform/nfc_hce.dart';
import 'pay_state.dart';

class PayPage extends ConsumerStatefulWidget {
  const PayPage({super.key});

  @override
  ConsumerState<PayPage> createState() => _PayPageState();
}

class _PayPageState extends ConsumerState<PayPage> {
  final _amountCtl = TextEditingController();
  final _memoCtl = TextEditingController();
  final _recipCtl = TextEditingController();
  bool _broadcasting = false;

  @override
  void dispose() {
    _amountCtl.dispose();
    _memoCtl.dispose();
    _recipCtl.dispose();
    super.dispose();
  }

  Future<void> _scanRecipient() async {
    final result = await Navigator.of(context).push<String>(
      MaterialPageRoute(builder: (_) => const _ScannerPage()),
    );
    if (result == null) return;
    setState(() => _recipCtl.text = result);
    ref.read(payDraftProvider.notifier).setRecipient(result);
  }

  @override
  Widget build(BuildContext context) {
    final draft = ref.watch(payDraftProvider);
    return Scaffold(
      appBar: AppBar(title: const Text('Pay')),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          TextField(
            controller: _recipCtl,
            decoration: InputDecoration(
              labelText: 'Recipient public key (hex)',
              suffixIcon: IconButton(
                icon: const Icon(Icons.qr_code_scanner),
                onPressed: _scanRecipient,
              ),
            ),
            onChanged: ref.read(payDraftProvider.notifier).setRecipient,
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _amountCtl,
            decoration: const InputDecoration(labelText: 'Amount (OWC)'),
            keyboardType: const TextInputType.numberWithOptions(decimal: true),
            onChanged: (v) => ref
                .read(payDraftProvider.notifier)
                .setAmount(double.tryParse(v) ?? 0),
          ),
          const SizedBox(height: 12),
          TextField(
            controller: _memoCtl,
            decoration: const InputDecoration(labelText: 'Memo'),
            onChanged: ref.read(payDraftProvider.notifier).setMemo,
          ),
          const SizedBox(height: 16),
          SegmentedButton<int>(
            segments: const [
              ButtonSegment(value: 1, label: Text('NFC'), icon: Icon(Icons.nfc)),
              ButtonSegment(
                  value: 2, label: Text('BLE'), icon: Icon(Icons.bluetooth)),
              ButtonSegment(
                  value: 3, label: Text('Online'), icon: Icon(Icons.cloud)),
            ],
            selected: {draft.channel},
            onSelectionChanged: (s) =>
                ref.read(payDraftProvider.notifier).setChannel(s.first),
          ),
          const SizedBox(height: 24),
          FilledButton.icon(
            onPressed: draft.recipientPublicKeyHex.isEmpty || draft.amountOwc <= 0
                ? null
                : () => _showSigned(context),
            icon: const Icon(Icons.lock),
            label: const Text('Sign and send'),
          ),
        ],
      ),
    );
  }

  Future<void> _showSigned(BuildContext context) async {
    final signed = await ref.read(signedPaymentProvider.future);
    if (!mounted) return;
    setState(() => _broadcasting = true);
    try {
      await showDialog(
        context: context,
        builder: (_) => _SignedDialog(signed: signed, channel: ref.read(payDraftProvider).channel),
      );
    } finally {
      if (mounted) setState(() => _broadcasting = false);
    }
  }
}

class _SignedDialog extends StatefulWidget {
  const _SignedDialog({required this.signed, required this.channel});

  final SignedPayment signed;
  final int channel;

  @override
  State<_SignedDialog> createState() => _SignedDialogState();
}

class _SignedDialogState extends State<_SignedDialog> {
  String _status = 'Ready';

  Future<void> _broadcastNfc() async {
    setState(() => _status = 'Broadcasting via NFC HCE…');
    try {
      await NfcHce.startSession(widget.signed.cbor);
      if (mounted) setState(() => _status = 'Tap recipient phone now.');
    } catch (e) {
      if (mounted) setState(() => _status = 'NFC failed: $e');
    }
  }

  @override
  void dispose() {
    NfcHce.endSession();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return AlertDialog(
      title: const Text('Signed payment'),
      content: SizedBox(
        width: 320,
        child: Column(
          mainAxisSize: MainAxisSize.min,
          children: [
            QrImageView(
              data: widget.signed.qrPayload,
              version: QrVersions.auto,
              size: 280,
            ),
            const SizedBox(height: 12),
            SelectableText(widget.signed.qrPayload,
                style: const TextStyle(fontFamily: 'monospace', fontSize: 11)),
            const SizedBox(height: 12),
            Text(_status),
          ],
        ),
      ),
      actions: [
        if (widget.channel == 1)
          TextButton(onPressed: _broadcastNfc, child: const Text('Send via NFC')),
        TextButton(
            onPressed: () => Navigator.of(context).pop(),
            child: const Text('Close')),
      ],
    );
  }
}

class _ScannerPage extends StatefulWidget {
  const _ScannerPage();

  @override
  State<_ScannerPage> createState() => _ScannerPageState();
}

class _ScannerPageState extends State<_ScannerPage> {
  final _ctrl = MobileScannerController();

  @override
  void dispose() {
    _ctrl.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Scan recipient')),
      body: MobileScanner(
        controller: _ctrl,
        onDetect: (capture) {
          for (final b in capture.barcodes) {
            final raw = b.rawValue;
            if (raw == null) continue;
            Navigator.of(context).pop(raw);
            return;
          }
        },
      ),
    );
  }
}
