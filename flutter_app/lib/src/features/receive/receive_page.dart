import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:mobile_scanner/mobile_scanner.dart';
import 'package:qr_flutter/qr_flutter.dart';

import '../../rust/api/transactions.dart';
import '../../rust/api/wire.dart';
import '../wallet/wallet_state.dart';

class ReceivePage extends ConsumerStatefulWidget {
  const ReceivePage({super.key});

  @override
  ConsumerState<ReceivePage> createState() => _ReceivePageState();
}

class _ReceivePageState extends ConsumerState<ReceivePage> {
  TransactionView? _decoded;
  String? _error;

  Future<void> _scanIncoming() async {
    final qr = await Navigator.of(context).push<String>(
      MaterialPageRoute(builder: (_) => const _ScannerPage()),
    );
    if (qr == null) return;
    try {
      final cbor = await qrDecode(qr: qr);
      final view = await decodeTransaction(cbor: cbor);
      setState(() {
        _decoded = view;
        _error = null;
      });
    } catch (e) {
      setState(() {
        _decoded = null;
        _error = '$e';
      });
    }
  }

  @override
  Widget build(BuildContext context) {
    final wallet = ref.watch(walletProvider).valueOrNull;
    return Scaffold(
      appBar: AppBar(
        title: const Text('Receive'),
        actions: [
          IconButton(
            tooltip: 'Scan incoming payment',
            icon: const Icon(Icons.qr_code_scanner),
            onPressed: _scanIncoming,
          ),
        ],
      ),
      body: ListView(
        padding: const EdgeInsets.all(16),
        children: [
          if (wallet != null) _PublicKeyCard(wallet: wallet),
          if (_decoded != null) _IncomingCard(view: _decoded!),
          if (_error != null)
            Card(
              color: Theme.of(context).colorScheme.errorContainer,
              child: Padding(
                padding: const EdgeInsets.all(16),
                child: Text('Could not decode payment: $_error'),
              ),
            ),
        ],
      ),
    );
  }
}

class _PublicKeyCard extends StatelessWidget {
  const _PublicKeyCard({required this.wallet});

  final Wallet wallet;

  String get _hex =>
      wallet.publicKey.map((b) => b.toRadixString(16).padLeft(2, '0')).join();

  @override
  Widget build(BuildContext context) {
    return Card(
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Your public key',
                style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 12),
            Center(
              child: QrImageView(
                data: _hex,
                version: QrVersions.auto,
                size: 240,
              ),
            ),
            const SizedBox(height: 12),
            Row(
              children: [
                Expanded(
                  child: SelectableText(
                    _hex,
                    style:
                        const TextStyle(fontFamily: 'monospace', fontSize: 11),
                  ),
                ),
                IconButton(
                  icon: const Icon(Icons.copy),
                  onPressed: () =>
                      Clipboard.setData(ClipboardData(text: _hex)),
                ),
              ],
            ),
          ],
        ),
      ),
    );
  }
}

class _IncomingCard extends StatelessWidget {
  const _IncomingCard({required this.view});

  final TransactionView view;

  @override
  Widget build(BuildContext context) {
    final amount = (view.amountMicroOwc.toInt() / 1000000).toStringAsFixed(2);
    return Card(
      color: view.signatureValid
          ? Theme.of(context).colorScheme.primaryContainer
          : Theme.of(context).colorScheme.errorContainer,
      child: Padding(
        padding: const EdgeInsets.all(16),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text(view.signatureValid ? 'Payment received' : 'Invalid signature',
                style: Theme.of(context).textTheme.titleMedium),
            const SizedBox(height: 8),
            _row('Amount', '$amount OWC'),
            _row('Memo', view.memo.isEmpty ? '—' : view.memo),
            _row('Channel',
                switch (view.channel) { 1 => 'NFC', 2 => 'BLE', 3 => 'Online', _ => '?' }),
            _row('Tx ID', view.transactionId),
          ],
        ),
      ),
    );
  }

  Widget _row(String k, String v) => Padding(
        padding: const EdgeInsets.symmetric(vertical: 4),
        child: Row(children: [
          SizedBox(width: 90, child: Text(k)),
          Expanded(child: Text(v, style: const TextStyle(fontWeight: FontWeight.w600))),
        ]),
      );
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
      appBar: AppBar(title: const Text('Scan payment')),
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
