import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';
import 'package:intl/intl.dart';

import 'wallet_state.dart';

class WalletHome extends ConsumerWidget {
  const WalletHome({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    final wallet = ref.watch(walletProvider);
    final balance = ref.watch(balanceProvider);

    return Scaffold(
      appBar: AppBar(title: const Text('Wallet')),
      body: wallet.when(
        loading: () => const Center(child: CircularProgressIndicator()),
        error: (e, _) => Center(child: Text('Failed to load wallet: $e')),
        data: (w) {
          if (w == null) {
            return const Center(child: Text('No wallet found.'));
          }
          return RefreshIndicator(
            onRefresh: () async {
              ref.invalidate(balanceProvider);
              await ref.read(balanceProvider.future);
            },
            child: ListView(
              padding: const EdgeInsets.all(16),
              children: [
                _BalanceCard(displayName: w.displayName, balance: balance),
                const SizedBox(height: 16),
                _QuickActions(),
                const SizedBox(height: 24),
                _AccountInfo(wallet: w),
              ],
            ),
          );
        },
      ),
    );
  }
}

class _BalanceCard extends StatelessWidget {
  const _BalanceCard({required this.displayName, required this.balance});

  final String displayName;
  final AsyncValue<int> balance;

  @override
  Widget build(BuildContext context) {
    final scheme = Theme.of(context).colorScheme;
    final fmt = NumberFormat.decimalPattern();
    return Card(
      color: scheme.primaryContainer,
      child: Padding(
        padding: const EdgeInsets.all(20),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Text('Hello, $displayName',
                style: Theme.of(context)
                    .textTheme
                    .titleMedium
                    ?.copyWith(color: scheme.onPrimaryContainer)),
            const SizedBox(height: 12),
            balance.when(
              loading: () => const SizedBox(
                  height: 40,
                  child: Center(child: CircularProgressIndicator())),
              error: (e, _) => Text('—',
                  style: Theme.of(context).textTheme.displaySmall),
              data: (b) => Row(
                crossAxisAlignment: CrossAxisAlignment.baseline,
                textBaseline: TextBaseline.alphabetic,
                children: [
                  Text(fmt.format(b ~/ 1000000),
                      style: Theme.of(context)
                          .textTheme
                          .displaySmall
                          ?.copyWith(
                              color: scheme.onPrimaryContainer,
                              fontWeight: FontWeight.w700)),
                  const SizedBox(width: 8),
                  Text('OWC',
                      style: TextStyle(color: scheme.onPrimaryContainer)),
                ],
              ),
            ),
            const SizedBox(height: 4),
            Text('Off-line capable balance, denominated in OWC.',
                style: Theme.of(context).textTheme.bodySmall?.copyWith(
                    color: scheme.onPrimaryContainer.withOpacity(0.8))),
          ],
        ),
      ),
    );
  }
}

class _QuickActions extends StatelessWidget {
  @override
  Widget build(BuildContext context) {
    return Row(
      children: [
        Expanded(
          child: FilledButton.icon(
            onPressed: () => context.go('/pay'),
            icon: const Icon(Icons.send),
            label: const Text('Pay'),
          ),
        ),
        const SizedBox(width: 12),
        Expanded(
          child: FilledButton.tonalIcon(
            onPressed: () => context.go('/receive'),
            icon: const Icon(Icons.qr_code),
            label: const Text('Receive'),
          ),
        ),
      ],
    );
  }
}

class _AccountInfo extends StatelessWidget {
  const _AccountInfo({required this.wallet});
  final Wallet wallet;

  @override
  Widget build(BuildContext context) {
    return Card(
      child: ListTile(
        title: const Text('Account ID'),
        subtitle: Text(wallet.userId,
            style: const TextStyle(fontFamily: 'monospace')),
        trailing: Text(_label(wallet.accountType)),
      ),
    );
  }

  String _label(String t) => switch (t) {
        'INDIVIDUAL' => 'Individual',
        'BUSINESS_POS' => 'POS',
        'BUSINESS_ELECTRONIC' => 'Online',
        _ => t,
      };
}
