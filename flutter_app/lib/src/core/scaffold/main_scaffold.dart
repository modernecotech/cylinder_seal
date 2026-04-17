import 'package:flutter/material.dart';
import 'package:go_router/go_router.dart';

class MainScaffold extends StatelessWidget {
  const MainScaffold({super.key, required this.child, required this.location});

  final Widget child;
  final String location;

  static const _destinations = <_Dest>[
    _Dest('/wallet', Icons.account_balance_wallet_outlined,
        Icons.account_balance_wallet, 'Wallet'),
    _Dest('/pay', Icons.send_outlined, Icons.send, 'Pay'),
    _Dest('/receive', Icons.qr_code_outlined, Icons.qr_code, 'Receive'),
    _Dest('/history', Icons.history_outlined, Icons.history, 'History'),
    _Dest('/business', Icons.storefront_outlined, Icons.storefront, 'Business'),
    _Dest('/settings', Icons.settings_outlined, Icons.settings, 'Settings'),
  ];

  int get _index =>
      _destinations.indexWhere((d) => location.startsWith(d.path)).clamp(0, _destinations.length - 1);

  @override
  Widget build(BuildContext context) {
    final wide = MediaQuery.sizeOf(context).width >= 720;
    if (wide) {
      return Scaffold(
        body: Row(children: [
          NavigationRail(
            selectedIndex: _index,
            onDestinationSelected: (i) => context.go(_destinations[i].path),
            labelType: NavigationRailLabelType.all,
            destinations: [
              for (final d in _destinations)
                NavigationRailDestination(
                  icon: Icon(d.icon),
                  selectedIcon: Icon(d.selectedIcon),
                  label: Text(d.label),
                ),
            ],
          ),
          const VerticalDivider(width: 1),
          Expanded(child: child),
        ]),
      );
    }
    return Scaffold(
      body: child,
      bottomNavigationBar: NavigationBar(
        selectedIndex: _index,
        onDestinationSelected: (i) => context.go(_destinations[i].path),
        destinations: [
          for (final d in _destinations)
            NavigationDestination(
              icon: Icon(d.icon),
              selectedIcon: Icon(d.selectedIcon),
              label: d.label,
            ),
        ],
      ),
    );
  }
}

class _Dest {
  const _Dest(this.path, this.icon, this.selectedIcon, this.label);
  final String path;
  final IconData icon;
  final IconData selectedIcon;
  final String label;
}
