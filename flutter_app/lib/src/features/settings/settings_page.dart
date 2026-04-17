import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import '../../core/storage/secure_store.dart';
import '../wallet/wallet_state.dart';

class SettingsPage extends ConsumerWidget {
  const SettingsPage({super.key});

  @override
  Widget build(BuildContext context, WidgetRef ref) {
    return Scaffold(
      appBar: AppBar(title: const Text('Settings')),
      body: ListView(
        children: [
          ListTile(
            leading: const Icon(Icons.delete_outline),
            title: const Text('Forget this device'),
            subtitle: const Text(
                'Wipes all keys and preferences. Only do this if you have a backup.'),
            onTap: () async {
              final confirm = await showDialog<bool>(
                context: context,
                builder: (ctx) => AlertDialog(
                  title: const Text('Forget this device?'),
                  content: const Text(
                      'All wallet data will be removed from this device. '
                      'Without a backup, the funds become unrecoverable.'),
                  actions: [
                    TextButton(
                        onPressed: () => Navigator.of(ctx).pop(false),
                        child: const Text('Cancel')),
                    FilledButton(
                        onPressed: () => Navigator.of(ctx).pop(true),
                        child: const Text('Forget')),
                  ],
                ),
              );
              if (confirm == true) {
                final store = ref.read(secureStoreProvider);
                final prefs =
                    await ref.read(userPreferencesProvider.future);
                await store.wipe();
                await prefs.reset();
                ref.invalidate(userPreferencesProvider);
                ref.invalidate(walletProvider);
              }
            },
          ),
        ],
      ),
    );
  }
}
