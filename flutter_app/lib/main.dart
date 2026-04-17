import 'package:flutter/material.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';

import 'src/app.dart';
import 'src/rust/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Loads the cs-bridge Rust library and wires up the Dart isolate.
  await RustLib.init();
  runApp(const ProviderScope(child: CylinderSealApp()));
}
