import 'dart:convert';

import 'package:flutter/foundation.dart';
import 'package:flutter_riverpod/flutter_riverpod.dart';
import 'package:flutter_secure_storage/flutter_secure_storage.dart';
import 'package:shared_preferences/shared_preferences.dart';

/// Secret material (private keys, mnemonic, hardware seed). Backed by
/// the platform Keystore on Android, Keychain on iOS, and IndexedDB
/// (encrypted-at-rest is not promised on the web target — flagged so the
/// caller can choose to require a fresh PIN-derived envelope before
/// trusting the web build with high-value funds).
class SecureStore {
  SecureStore(this._inner);
  final FlutterSecureStorage _inner;

  static const _privateKey = 'cs.privkey.v1';
  static const _publicKey = 'cs.pubkey.v1';
  static const _hardwareSeed = 'cs.hwseed.v1';
  static const _lastNonce = 'cs.lastnonce.v1';

  Future<void> writePrivateKey(Uint8List bytes) =>
      _inner.write(key: _privateKey, value: base64Encode(bytes));

  Future<Uint8List?> readPrivateKey() async {
    final raw = await _inner.read(key: _privateKey);
    return raw == null ? null : base64Decode(raw);
  }

  Future<void> writePublicKey(Uint8List bytes) =>
      _inner.write(key: _publicKey, value: base64Encode(bytes));

  Future<Uint8List?> readPublicKey() async {
    final raw = await _inner.read(key: _publicKey);
    return raw == null ? null : base64Decode(raw);
  }

  Future<void> writeHardwareSeed(Uint8List bytes) =>
      _inner.write(key: _hardwareSeed, value: base64Encode(bytes));

  Future<Uint8List?> readHardwareSeed() async {
    final raw = await _inner.read(key: _hardwareSeed);
    return raw == null ? null : base64Decode(raw);
  }

  Future<void> writeLastNonce(Uint8List bytes) =>
      _inner.write(key: _lastNonce, value: base64Encode(bytes));

  Future<Uint8List?> readLastNonce() async {
    final raw = await _inner.read(key: _lastNonce);
    return raw == null ? null : base64Decode(raw);
  }

  Future<void> wipe() => _inner.deleteAll();
}

final secureStoreProvider = Provider<SecureStore>((ref) {
  // Web has no hardware-backed keychain — caller must layer a PIN-derived
  // wrap on top before trusting it for production use.
  if (kIsWeb) {
    return SecureStore(const FlutterSecureStorage());
  }
  return SecureStore(const FlutterSecureStorage(
    aOptions: AndroidOptions(encryptedSharedPreferences: true),
    iOptions: IOSOptions(accessibility: KeychainAccessibility.first_unlock),
  ));
});

/// Non-secret prefs (display name, superpeer host, last-sync-at).
class UserPreferences {
  UserPreferences(this._prefs);
  final SharedPreferences _prefs;

  static const _onboarded = 'onboarded';
  static const _displayName = 'display_name';
  static const _phone = 'phone_number';
  static const _kycTier = 'kyc_tier';
  static const _superpeerHost = 'superpeer_host';
  static const _superpeerPort = 'superpeer_port';
  static const _lastSyncAt = 'last_sync_at';
  static const _accountType = 'account_type';
  static const _userId = 'user_id';

  static const String defaultHost = 'sp-baghdad.cbi.iq';
  static const int defaultPort = 50051;

  bool get isOnboarded => _prefs.getBool(_onboarded) ?? false;
  String? get displayName => _prefs.getString(_displayName);
  String? get phoneNumber => _prefs.getString(_phone);
  String get kycTier => _prefs.getString(_kycTier) ?? 'ANONYMOUS';
  String get superpeerHost => _prefs.getString(_superpeerHost) ?? defaultHost;
  int get superpeerPort => _prefs.getInt(_superpeerPort) ?? defaultPort;
  DateTime get lastSyncAt =>
      DateTime.fromMillisecondsSinceEpoch(_prefs.getInt(_lastSyncAt) ?? 0);
  String get accountType => _prefs.getString(_accountType) ?? 'INDIVIDUAL';
  String? get userId => _prefs.getString(_userId);

  Future<void> completeOnboarding({
    required String displayName,
    String? phoneNumber,
    required String accountType,
    required String userId,
  }) async {
    await _prefs.setBool(_onboarded, true);
    await _prefs.setString(_displayName, displayName);
    if (phoneNumber != null) await _prefs.setString(_phone, phoneNumber);
    await _prefs.setString(_accountType, accountType);
    await _prefs.setString(_userId, userId);
  }

  Future<void> setSuperpeer(String host, int port) async {
    await _prefs.setString(_superpeerHost, host);
    await _prefs.setInt(_superpeerPort, port);
  }

  Future<void> recordSync(DateTime now) =>
      _prefs.setInt(_lastSyncAt, now.millisecondsSinceEpoch);

  Future<void> reset() => _prefs.clear();
}

final userPreferencesProvider = FutureProvider<UserPreferences>((ref) async {
  final prefs = await SharedPreferences.getInstance();
  return UserPreferences(prefs);
});
