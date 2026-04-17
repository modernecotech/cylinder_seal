import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:go_router/go_router.dart';

import '../../features/business/business_dashboard.dart';
import '../../features/history/history_page.dart';
import '../../features/onboarding/onboarding_flow.dart';
import '../../features/onboarding/onboarding_state.dart';
import '../../features/pay/pay_page.dart';
import '../../features/receive/receive_page.dart';
import '../../features/settings/settings_page.dart';
import '../../features/wallet/wallet_home.dart';
import '../scaffold/main_scaffold.dart';

final appRouterProvider = Provider<GoRouter>((ref) {
  final isOnboarded = ref.watch(isOnboardedProvider);
  return GoRouter(
    initialLocation: '/wallet',
    redirect: (ctx, state) {
      final loc = state.matchedLocation;
      final atOnboarding = loc.startsWith('/onboarding');
      if (!isOnboarded && !atOnboarding) return '/onboarding';
      if (isOnboarded && atOnboarding) return '/wallet';
      return null;
    },
    routes: [
      GoRoute(
        path: '/onboarding',
        builder: (_, __) => const OnboardingFlow(),
      ),
      ShellRoute(
        builder: (ctx, state, child) =>
            MainScaffold(location: state.matchedLocation, child: child),
        routes: [
          GoRoute(path: '/wallet', builder: (_, __) => const WalletHome()),
          GoRoute(path: '/pay', builder: (_, __) => const PayPage()),
          GoRoute(path: '/receive', builder: (_, __) => const ReceivePage()),
          GoRoute(path: '/history', builder: (_, __) => const HistoryPage()),
          GoRoute(path: '/business', builder: (_, __) => const BusinessDashboard()),
          GoRoute(path: '/settings', builder: (_, __) => const SettingsPage()),
        ],
      ),
    ],
  );
});
