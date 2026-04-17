import 'package:flutter/material.dart';

class BusinessDashboard extends StatelessWidget {
  const BusinessDashboard({super.key});

  @override
  Widget build(BuildContext context) {
    return Scaffold(
      appBar: AppBar(title: const Text('Business')),
      body: const Center(
        child: Text('Business dashboard — implemented in Session 3.'),
      ),
    );
  }
}
