import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'onboarding_state.dart';

class OnboardingFlow extends ConsumerStatefulWidget {
  const OnboardingFlow({super.key});

  @override
  ConsumerState<OnboardingFlow> createState() => _OnboardingFlowState();
}

class _OnboardingFlowState extends ConsumerState<OnboardingFlow> {
  final _pageCtrl = PageController();
  bool _submitting = false;

  void _next() => _pageCtrl.nextPage(
      duration: const Duration(milliseconds: 250), curve: Curves.easeOut);

  Future<void> _finish() async {
    setState(() => _submitting = true);
    try {
      await ref.read(onboardingControllerProvider.notifier).submit();
    } catch (e) {
      if (mounted) {
        ScaffoldMessenger.of(context).showSnackBar(
          SnackBar(content: Text('Onboarding failed: $e')),
        );
        setState(() => _submitting = false);
      }
    }
  }

  @override
  Widget build(BuildContext context) {
    final form = ref.watch(onboardingControllerProvider);
    final ctrl = ref.read(onboardingControllerProvider.notifier);
    final isBusiness = form.accountType.startsWith('BUSINESS');

    return Scaffold(
      body: SafeArea(
        child: PageView(
          controller: _pageCtrl,
          physics: const NeverScrollableScrollPhysics(),
          children: [
            _WelcomePage(onContinue: _next),
            _AccountTypePage(
              selected: form.accountType,
              onSelect: ctrl.setAccountType,
              onContinue: _next,
            ),
            _IdentityPage(
              isBusiness: isBusiness,
              form: form,
              onName: ctrl.setDisplayName,
              onPhone: ctrl.setPhone,
              onBusinessName: ctrl.setBusinessName,
              onBusinessTaxId: ctrl.setBusinessTaxId,
              onContinue: _next,
            ),
            _ConfirmPage(
              submitting: _submitting,
              form: form,
              onConfirm: _finish,
            ),
          ],
        ),
      ),
    );
  }
}

class _WelcomePage extends StatelessWidget {
  const _WelcomePage({required this.onContinue});
  final VoidCallback onContinue;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24),
      child: Column(
        mainAxisAlignment: MainAxisAlignment.center,
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const Icon(Icons.account_balance, size: 96),
          const SizedBox(height: 24),
          Text('Cylinder Seal',
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.headlineMedium),
          const SizedBox(height: 8),
          Text('Digital Iraqi Dinar wallet',
              textAlign: TextAlign.center,
              style: Theme.of(context).textTheme.bodyLarge),
          const Spacer(),
          FilledButton(onPressed: onContinue, child: const Text('Get started')),
        ],
      ),
    );
  }
}

class _AccountTypePage extends StatelessWidget {
  const _AccountTypePage({
    required this.selected,
    required this.onSelect,
    required this.onContinue,
  });

  final String selected;
  final ValueChanged<String> onSelect;
  final VoidCallback onContinue;

  static const _options = [
    ('INDIVIDUAL', 'Individual', 'Personal wallet for daily payments.'),
    ('BUSINESS_POS', 'Business — POS', 'Brick-and-mortar shop accepting in-person payments.'),
    ('BUSINESS_ELECTRONIC', 'Business — Online', 'Online merchant issuing invoices via API.'),
  ];

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const SizedBox(height: 16),
          Text('Choose your account type',
              style: Theme.of(context).textTheme.headlineSmall),
          const SizedBox(height: 16),
          for (final (id, label, desc) in _options)
            Card(
              child: RadioListTile<String>(
                value: id,
                groupValue: selected,
                onChanged: (v) => v != null ? onSelect(v) : null,
                title: Text(label),
                subtitle: Text(desc),
              ),
            ),
          const Spacer(),
          FilledButton(onPressed: onContinue, child: const Text('Continue')),
        ],
      ),
    );
  }
}

class _IdentityPage extends StatelessWidget {
  const _IdentityPage({
    required this.isBusiness,
    required this.form,
    required this.onName,
    required this.onPhone,
    required this.onBusinessName,
    required this.onBusinessTaxId,
    required this.onContinue,
  });

  final bool isBusiness;
  final OnboardingForm form;
  final ValueChanged<String> onName;
  final ValueChanged<String> onPhone;
  final ValueChanged<String> onBusinessName;
  final ValueChanged<String> onBusinessTaxId;
  final VoidCallback onContinue;

  @override
  Widget build(BuildContext context) {
    final canContinue = form.displayName.trim().isNotEmpty &&
        (!isBusiness || (form.businessName ?? '').trim().isNotEmpty);
    return Padding(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const SizedBox(height: 16),
          Text('Tell us about yourself',
              style: Theme.of(context).textTheme.headlineSmall),
          const SizedBox(height: 24),
          TextField(
            decoration: const InputDecoration(labelText: 'Display name'),
            onChanged: onName,
          ),
          const SizedBox(height: 16),
          TextField(
            decoration: const InputDecoration(
                labelText: 'Phone number (optional, for KYC tier-up)'),
            keyboardType: TextInputType.phone,
            onChanged: onPhone,
          ),
          if (isBusiness) ...[
            const SizedBox(height: 24),
            const Divider(),
            const SizedBox(height: 16),
            TextField(
              decoration: const InputDecoration(labelText: 'Business name'),
              onChanged: onBusinessName,
            ),
            const SizedBox(height: 16),
            TextField(
              decoration:
                  const InputDecoration(labelText: 'Tax ID / commercial reg #'),
              onChanged: onBusinessTaxId,
            ),
          ],
          const Spacer(),
          FilledButton(
            onPressed: canContinue ? onContinue : null,
            child: const Text('Continue'),
          ),
        ],
      ),
    );
  }
}

class _ConfirmPage extends StatelessWidget {
  const _ConfirmPage({
    required this.submitting,
    required this.form,
    required this.onConfirm,
  });

  final bool submitting;
  final OnboardingForm form;
  final VoidCallback onConfirm;

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.all(24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.stretch,
        children: [
          const SizedBox(height: 16),
          Text('Confirm and create wallet',
              style: Theme.of(context).textTheme.headlineSmall),
          const SizedBox(height: 24),
          Card(
            child: Padding(
              padding: const EdgeInsets.all(16),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  _row('Account type', _accountLabel(form.accountType)),
                  _row('Display name', form.displayName),
                  if ((form.phoneNumber ?? '').isNotEmpty)
                    _row('Phone', form.phoneNumber!),
                  if ((form.businessName ?? '').isNotEmpty)
                    _row('Business', form.businessName!),
                  if ((form.businessTaxId ?? '').isNotEmpty)
                    _row('Tax ID', form.businessTaxId!),
                ],
              ),
            ),
          ),
          const SizedBox(height: 16),
          Text(
            'Your private key is generated on-device and stored in the platform '
            'keystore. It never leaves this phone. Losing this device without a '
            'backup means losing the wallet.',
            style: Theme.of(context).textTheme.bodySmall,
          ),
          const Spacer(),
          FilledButton(
            onPressed: submitting ? null : onConfirm,
            child: submitting
                ? const SizedBox(
                    width: 20,
                    height: 20,
                    child: CircularProgressIndicator(strokeWidth: 2.5))
                : const Text('Create wallet'),
          ),
        ],
      ),
    );
  }

  Widget _row(String k, String v) => Padding(
        padding: const EdgeInsets.symmetric(vertical: 4),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(width: 120, child: Text(k)),
            Expanded(
                child:
                    Text(v, style: const TextStyle(fontWeight: FontWeight.w600))),
          ],
        ),
      );

  String _accountLabel(String t) => switch (t) {
        'INDIVIDUAL' => 'Individual',
        'BUSINESS_POS' => 'Business — POS',
        'BUSINESS_ELECTRONIC' => 'Business — Online',
        _ => t,
      };
}
