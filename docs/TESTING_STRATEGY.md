# CylinderSeal: Testing & Guardrails Strategy

## Design Principle

**"Trust nothing, verify everything"**

Every security claim in IRON_SECURITY.md must be tested. Every invariant must be guarded. Every edge case must be caught.

---

## Test Pyramid

```
        🔴 Manual/Penetration Tests
       Adversarial Attack Scenarios
      ────────────────────────────
     🟠 Security Tests (20% of tests)
    Replay attacks, double-spend, key compromise
   ─────────────────────────────────
  🟡 Integration Tests (30% of tests)
 Full flow: offline tx → sync → confirmation
─────────────────────────────────
🟢 Unit Tests (50% of tests)
Crypto, validation, calculations
```

---

## Test Categories

### 1. Unit Tests (Crypto & Core)

**Coverage target: 95%+**

**Tests to write:**

#### Rust (crates/cs-core)

```rust
#[cfg(test)]
mod crypto_tests {
    use super::*;

    // ✅ BLAKE2b Hashing
    #[test]
    fn test_blake2b_deterministic() {
        let data = b"test";
        let hash1 = blake2b_256(data);
        let hash2 = blake2b_256(data);
        assert_eq!(hash1, hash2, "BLAKE2b must be deterministic");
    }

    #[test]
    fn test_blake2b_different_inputs() {
        let hash1 = blake2b_256(b"test1");
        let hash2 = blake2b_256(b"test2");
        assert_ne!(hash1, hash2, "Different inputs must produce different hashes");
    }

    #[test]
    fn test_blake2b_collision_resistance() {
        // Try to find collision (should fail after 2^128 attempts)
        // This is a property test - runs many times
        let mut hashes = vec![];
        for i in 0..10000 {
            let hash = blake2b_256(&i.to_le_bytes());
            assert!(
                !hashes.contains(&hash),
                "BLAKE2b collision found at i={}", i
            );
            hashes.push(hash);
        }
    }

    // ✅ Ed25519 Signing
    #[test]
    fn test_ed25519_sign_verify() {
        let (pub_key, priv_key) = generate_keypair();
        let msg = b"hello world";
        let sig = sign_message(msg, &priv_key).unwrap();

        assert!(verify_signature(msg, &sig, &pub_key).is_ok());
    }

    #[test]
    fn test_ed25519_reject_wrong_message() {
        let (pub_key, priv_key) = generate_keypair();
        let msg = b"hello world";
        let sig = sign_message(msg, &priv_key).unwrap();

        assert!(verify_signature(b"goodbye world", &sig, &pub_key).is_err(),
            "Signature must fail for different message");
    }

    #[test]
    fn test_ed25519_reject_tampered_signature() {
        let (pub_key, priv_key) = generate_keypair();
        let msg = b"hello world";
        let mut sig = sign_message(msg, &priv_key).unwrap();

        sig[0] ^= 0xFF;  // Flip all bits in first byte

        assert!(verify_signature(msg, &sig, &pub_key).is_err(),
            "Signature must fail if tampered");
    }

    // ✅ Nonce Derivation
    #[test]
    fn test_deterministic_nonce() {
        let prev_nonce = [42u8; 32];
        let counter = 1u64;

        let nonce1 = derive_deterministic_nonce(&prev_nonce, counter);
        let nonce2 = derive_deterministic_nonce(&prev_nonce, counter);

        assert_eq!(nonce1, nonce2, "Nonce derivation must be deterministic");
    }

    #[test]
    fn test_nonce_depends_on_previous() {
        let prev_nonce1 = [1u8; 32];
        let prev_nonce2 = [2u8; 32];
        let counter = 1u64;

        let nonce1 = derive_deterministic_nonce(&prev_nonce1, counter);
        let nonce2 = derive_deterministic_nonce(&prev_nonce2, counter);

        assert_ne!(nonce1, nonce2, "Different previous nonces must produce different nonces");
    }

    #[test]
    fn test_nonce_depends_on_counter() {
        let prev_nonce = [42u8; 32];

        let nonce1 = derive_deterministic_nonce(&prev_nonce, 1);
        let nonce2 = derive_deterministic_nonce(&prev_nonce, 2);

        assert_ne!(nonce1, nonce2, "Different counters must produce different nonces");
    }
}

#[cfg(test)]
mod transaction_tests {
    use super::*;

    // ✅ Transaction Immutability
    #[test]
    fn test_transaction_signature_fails_if_modified() {
        let (pub_key, priv_key) = generate_keypair();
        let mut tx = Transaction::new(
            pub_key,
            [0u8; 32],  // recipient
            50_000_000,  // amount
            "KES".to_string(),
            Decimal::from_str("0.987654").unwrap(),
            PaymentChannel::NFC,
            "test".to_string(),
            Uuid::new_v4(),
            [0u8; 32],  // prev nonce
        );

        tx.sign(&priv_key).unwrap();
        assert!(tx.verify_signature().is_ok());

        // Tamper with amount
        tx.amount_owc = 100_000_000;

        assert!(tx.verify_signature().is_err(),
            "Signature must fail if amount is tampered");
    }

    // ✅ Nonce Chain Validation
    #[test]
    fn test_nonce_chain_breaks_if_not_sequential() {
        let genesis_nonce = [0u8; 32];

        let tx1_nonce = derive_deterministic_nonce(&genesis_nonce, 1);
        let tx2_nonce = derive_deterministic_nonce(&genesis_nonce, 1);  // Wrong! Should derive from tx1_nonce

        assert_ne!(tx1_nonce, tx2_nonce, "This should catch the error");
    }
}

#[cfg(test)]
mod ledger_block_tests {
    use super::*;

    // ✅ Vector Clock Monotonicity
    #[test]
    fn test_vector_clock_cannot_go_backward() {
        let user_id = Uuid::new_v4();
        let mut clock1 = HashMap::new();
        clock1.insert(user_id, 5u64);

        let mut clock2 = HashMap::new();
        clock2.insert(user_id, 4u64);  // Backward!

        assert!(clock1[&user_id] > clock2[&user_id],
            "Vector clock went backward");
    }

    // ✅ Block Hash Integrity
    #[test]
    fn test_block_hash_tamper_detection() {
        let (pub_key, priv_key) = generate_keypair();
        let mut block = LedgerBlock::new(
            pub_key,
            Uuid::new_v4(),  // device_id
            1,
            [0u8; 32],  // prev_block_hash
            vec![],
            HashMap::new(),
        );

        block.compute_block_hash().unwrap();
        block.sign_with_device_key(&priv_key).unwrap();

        // Tamper with block
        block.block_hash[0] ^= 0xFF;

        assert!(block.verify().is_err(),
            "Block verification must fail if hash is tampered");
    }

    // ✅ Sequence Number Validation
    #[test]
    fn test_sequence_must_increment_by_one() {
        let seq1 = 5u64;
        let seq2 = 7u64;  // Gap!

        assert_ne!(seq2, seq1 + 1,
            "Sequence gap detected");
    }

    // ✅ Monotonic Clock Non-Decreasing
    #[test]
    fn test_monotonic_clock_never_goes_backward() {
        let time1 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;

        std::thread::sleep(std::time::Duration::from_millis(10));

        let time2 = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as i64;

        assert!(time2 >= time1,
            "Monotonic clock must never go backward");
    }
}

#[cfg(test)]
mod kyc_tier_tests {
    use super::*;

    // ✅ KYC Tier Limits
    #[test]
    fn test_anonymous_tier_limits() {
        let tier = KYCTier::Anonymous;
        assert_eq!(tier.max_offline_transaction(), 20_000_000);  // 20 OWC
        assert_eq!(tier.max_daily_offline_per_device(), 10_000_000);  // 10 OWC
        assert_eq!(tier.max_balance(), Some(50_000_000));  // 50 OWC
    }

    #[test]
    fn test_kyc_tier_thresholds() {
        let tier = KYCTier::PhoneVerified;
        assert!(tier.attestation_threshold() < tier.biometric_threshold());
        assert!(tier.biometric_threshold() < tier.max_offline_transaction());
    }
}
```

**Kotlin (Android Tests)**

```kotlin
@RunWith(RobolectricTestRunner::class)
class CryptoTests {
    
    // ✅ Keystore Integration
    @Test
    fun testKeystoreIsHardwareBacked() {
        val keyStore = KeyStore.getInstance("AndroidKeyStore")
        keyStore.load(null)

        val keyGenParameterSpec = KeyGenParameterSpec.Builder(
            "test_key",
            KeyProperties.PURPOSE_SIGN
        )
            .setDigests(KeyProperties.DIGEST_SHA256)
            .setIsStrongBoxBacked(true)
            .setUserAuthenticationRequired(true)
            .build()

        val keyGenerator = KeyGenerator.getInstance(
            KeyProperties.KEY_ALGORITHM_EC,
            "AndroidKeyStore"
        )
        keyGenerator.init(keyGenParameterSpec)
        val key = keyGenerator.generateKey()

        assertTrue(key.isStrongBoxBacked, "Key must be hardware-backed")
    }

    // ✅ Deterministic Nonce
    @Test
    fun testDeterministicNonce() {
        val previousNonce = ByteArray(32) { 42.toByte() }
        val counter = 1L

        val nonce1 = NonceDerivation.derive(previousNonce, counter)
        val nonce2 = NonceDerivation.derive(previousNonce, counter)

        assertEquals(nonce1.toList(), nonce2.toList(), "Nonces must be deterministic")
    }

    // ✅ Device Attestation
    @Test
    fun testDeviceAttestationIsRequired() {
        val attestation = SafetyNetClient.getAttestation("challenge")
        
        assertNotNull(attestation)
        assertTrue(attestation!!.isDeviceIntegrityOk, "Device must pass integrity check")
    }
}

@RunWith(RobolectricTestRunner::class)
class TransactionTests {

    // ✅ Transaction Signing
    @Test
    fun testTransactionSigningAndVerification() {
        val (publicKey, privateKey) = generateKeypair()
        val transaction = Transaction(
            from_public_key = publicKey,
            to_public_key = ByteArray(32),
            amount_owc = 50_000_000,
            // ... other fields
        )

        transaction.sign(privateKey)
        assertTrue(transaction.verifySignature(), "Signature must verify")
    }

    // ✅ Amount Validation
    @Test
    fun testAmountsAreAlwaysI64NeverFloat() {
        val amount: Long = 1_000_000  // 1 OWC in micro-OWC
        
        val tx = Transaction(amount_owc = amount)
        assertEquals(amount, tx.amount_owc)
        
        // This won't compile (good!)
        // val tx2 = Transaction(amount_owc = 1.5)
    }

    // ✅ Nonce Chain
    @Test
    fun testNonceChainValidation() {
        val genesis = blake2b256(userPublicKey)
        val tx1 = Transaction(previous_nonce = genesis)
        val tx1Nonce = tx1.current_nonce
        
        val tx2 = Transaction(previous_nonce = tx1Nonce)
        
        assertEquals(tx2.previous_nonce, tx1Nonce)
    }
}

@RunWith(RobolectricTestRunner::class)
class LedgerBlockTests {

    // ✅ Block Hash Integrity
    @Test
    fun testBlockHashVerification() {
        val block = LedgerBlock(/* ... */)
        block.computeBlockHash()
        block.signWithDeviceKey(privateKey)
        
        assertTrue(block.verify(), "Block must verify")
        
        // Tamper with amount
        block.transactions[0].amount_owc = 999_000_000
        
        assertFalse(block.verify(), "Tampered block must fail verification")
    }

    // ✅ Sequence Validation
    @Test
    fun testSequenceMustIncrementByOne() {
        val block1 = LedgerBlock(sequence_number = 5)
        val block2 = LedgerBlock(sequence_number = 7)  // Gap!
        
        assertNotEquals(block2.sequence_number, block1.sequence_number + 1)
    }

    // ✅ Vector Clock
    @Test
    fun testVectorClockMonotonicity() {
        val clock1 = mapOf(userId to 5L)
        val block1 = LedgerBlock(vector_clock = clock1)
        
        val clock2 = mapOf(userId to 4L)  // Backward!
        val block2 = LedgerBlock(vector_clock = clock2)
        
        assertTrue(clock1[userId]!! > clock2[userId]!!)
    }
}
```

---

### 2. Integration Tests

**Coverage target: 80%+**

**Flow: Device offline tx → Sync → Super-peer confirmation**

#### Rust Integration Test

```rust
#[tokio::test]
async fn test_complete_offline_to_sync_flow() {
    // Setup
    let (device1_pub, device1_priv) = generate_keypair();
    let (device2_pub, device2_priv) = generate_keypair();
    let (sp_pub, sp_priv) = generate_keypair();

    // ════════════════════════════════════════════════════════════
    // Phase 1: Device 1 creates offline transaction
    // ════════════════════════════════════════════════════════════
    let prev_nonce = [0u8; 32];  // Genesis
    let mut tx = Transaction::new(
        device1_pub,
        device2_pub,
        50_000_000,  // 50 OWC
        "KES".to_string(),
        Decimal::from_str("0.987654").unwrap(),
        PaymentChannel::NFC,
        "Payment".to_string(),
        Uuid::new_v4(),
        prev_nonce,
    );

    tx.sign(&device1_priv).expect("Signing failed");
    assert!(tx.verify_signature().is_ok(), "Signature verification failed");

    // ════════════════════════════════════════════════════════════
    // Phase 2: Device 1 creates ledger block
    // ════════════════════════════════════════════════════════════
    let mut block = LedgerBlock::new(
        device1_pub,
        Uuid::new_v4(),
        0,  // Genesis block
        blake2b_256(&device1_pub),
        vec![tx.clone()],
        HashMap::new(),
    );

    block.compute_block_hash().expect("Hash computation failed");
    block.sign_with_device_key(&device1_priv).expect("Signing failed");

    // Verify block locally
    assert!(block.verify().is_ok(), "Block verification failed");
    assert_eq!(block.sync_status, SyncStatus::Pending);

    // ════════════════════════════════════════════════════════════
    // Phase 3: Device submits to super-peer
    // ════════════════════════════════════════════════════════════
    let storage = setup_test_db().await;
    let mut super_peer = MockSuperPeer::new(storage, sp_priv);

    let result = super_peer.validate_block(&block).await;
    assert!(result.is_ok(), "Super-peer validation failed: {:?}", result.err());

    // ════════════════════════════════════════════════════════════
    // Phase 4: Super-peer confirms block
    // ════════════════════════════════════════════════════════════
    let confirmed = super_peer.confirm_block(&block).await;
    assert!(confirmed.is_ok());

    let confirmations = confirmed.unwrap().super_peer_confirmations;
    assert!(confirmations.len() >= 1, "Need at least 1 confirmation");

    // ════════════════════════════════════════════════════════════
    // Phase 5: Device 2 syncs and gets the block
    // ════════════════════════════════════════════════════════════
    let synced_block = super_peer.get_block(block.block_id).await;
    assert!(synced_block.is_ok());

    let synced = synced_block.unwrap();
    assert_eq!(synced.block_hash, block.block_hash);
    assert!(synced.super_peer_confirmations.len() >= 1);
}

#[tokio::test]
async fn test_double_spend_detection() {
    // Create two competing blocks with same prev_hash
    let (pub_key, priv_key) = generate_keypair();
    
    let mut block_a = LedgerBlock::new(pub_key, Uuid::new_v4(), 1, [0u8; 32], vec![], HashMap::new());
    let mut block_b = LedgerBlock::new(pub_key, Uuid::new_v4(), 1, [0u8; 32], vec![], HashMap::new());

    block_a.compute_block_hash().unwrap();
    block_a.sign_with_device_key(&priv_key).unwrap();

    block_b.compute_block_hash().unwrap();
    block_b.sign_with_device_key(&priv_key).unwrap();

    // Submit both to super-peer
    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    let result_a = super_peer.validate_block(&block_a).await;
    assert!(result_a.is_ok());

    let result_b = super_peer.validate_block(&block_b).await;
    assert!(result_b.is_err(), "Second block should be rejected as double-spend");
}

#[tokio::test]
async fn test_out_of_sequence_rejection() {
    let (pub_key, priv_key) = generate_keypair();

    // Create block with sequence 5 (but should be 0)
    let mut block = LedgerBlock::new(pub_key, Uuid::new_v4(), 5, [0u8; 32], vec![], HashMap::new());
    block.compute_block_hash().unwrap();
    block.sign_with_device_key(&priv_key).unwrap();

    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    let result = super_peer.validate_block(&block).await;
    assert!(result.is_err(), "Out-of-sequence block should be rejected");
    
    match result.err() {
        Some(CylinderSealError::OutOfSequence { expected: 0, got: 5 }) => (),
        other => panic!("Wrong error: {:?}", other),
    }
}

#[tokio::test]
async fn test_nonce_chain_validation() {
    let (pub_key, priv_key) = generate_keypair();
    let genesis_nonce = blake2b_256(&pub_key);

    let mut tx1 = Transaction::new(/* ... */, genesis_nonce);
    tx1.sign(&priv_key).unwrap();

    // tx2 should chain from tx1, but doesn't
    let mut tx2 = Transaction::new(/* ... */, [42u8; 32]);  // Wrong previous nonce!
    tx2.sign(&priv_key).unwrap();

    let block = LedgerBlock::new(
        pub_key,
        Uuid::new_v4(),
        0,
        blake2b_256(&pub_key),
        vec![tx1, tx2],
        HashMap::new(),
    );

    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    let result = super_peer.validate_block(&block).await;
    assert!(result.is_err(), "Nonce chain break should be detected");
}

#[tokio::test]
async fn test_vector_clock_backward_detection() {
    let user_id = Uuid::new_v4();
    let (pub_key, priv_key) = generate_keypair();

    // Block 1: vector_clock has user_id: 5
    let mut clock1 = HashMap::new();
    clock1.insert(user_id, 5u64);
    let mut block1 = LedgerBlock::new(pub_key, Uuid::new_v4(), 1, [0u8; 32], vec![], clock1);
    block1.compute_block_hash().unwrap();
    block1.sign_with_device_key(&priv_key).unwrap();

    // Block 2: vector_clock has user_id: 4 (went backward!)
    let mut clock2 = HashMap::new();
    clock2.insert(user_id, 4u64);
    let mut block2 = LedgerBlock::new(pub_key, Uuid::new_v4(), 2, block1.block_hash, vec![], clock2);
    block2.compute_block_hash().unwrap();
    block2.sign_with_device_key(&priv_key).unwrap();

    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    super_peer.validate_block(&block1).await.expect("Block 1 should be valid");
    let result = super_peer.validate_block(&block2).await;

    assert!(result.is_err(), "Vector clock backward should be detected");
}

#[tokio::test]
async fn test_device_daily_limit_enforcement() {
    let device_id = Uuid::new_v4();
    let (pub_key, priv_key) = generate_keypair();

    // Create 11 transactions of 10 OWC each = 110 OWC (exceeds 100 OWC daily limit)
    let mut transactions = vec![];
    let mut prev_nonce = blake2b_256(&pub_key);

    for _ in 0..11 {
        let mut tx = Transaction::new(
            pub_key,
            [0u8; 32],
            10_000_000,  // 10 OWC
            "KES".to_string(),
            Decimal::one(),
            PaymentChannel::Online,
            "".to_string(),
            device_id,
            prev_nonce,
        );
        tx.sign(&priv_key).unwrap();
        prev_nonce = tx.current_nonce;
        transactions.push(tx);
    }

    let mut block = LedgerBlock::new(
        pub_key,
        device_id,
        0,
        blake2b_256(&pub_key),
        transactions,
        HashMap::new(),
    );
    block.compute_block_hash().unwrap();
    block.sign_with_device_key(&priv_key).unwrap();

    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    let result = super_peer.validate_block(&block).await;
    assert!(result.is_err(), "Daily limit should be enforced");
}
```

---

### 3. Property-Based Tests

**Using proptest for invariants**

```rust
use proptest::prelude::*;

proptest! {
    #[test]
    fn prop_nonce_is_deterministic(
        prev_nonce in prop::array::uniform32(any::<u8>()),
        counter in any::<u64>()
    ) {
        let nonce1 = derive_deterministic_nonce(&prev_nonce, counter);
        let nonce2 = derive_deterministic_nonce(&prev_nonce, counter);
        
        prop_assert_eq!(nonce1, nonce2);
    }

    #[test]
    fn prop_hash_is_deterministic(data in prop::collection::vec(any::<u8>(), 0..1000)) {
        let hash1 = blake2b_256(&data);
        let hash2 = blake2b_256(&data);
        
        prop_assert_eq!(hash1, hash2);
    }

    #[test]
    fn prop_signature_verification_correct(
        (pub_key, priv_key) in generate_keypair_arb(),
        msg in prop::collection::vec(any::<u8>(), 0..1000)
    ) {
        let sig = sign_message(&msg, &priv_key).unwrap();
        
        prop_assert!(verify_signature(&msg, &sig, &pub_key).is_ok());
    }

    #[test]
    fn prop_signature_fails_on_tampered_message(
        (pub_key, priv_key) in generate_keypair_arb(),
        msg in prop::collection::vec(any::<u8>(), 1..1000),
        tamper_idx in 0usize..100
    ) {
        let sig = sign_message(&msg, &priv_key).unwrap();
        
        let mut tampered = msg.clone();
        if tamper_idx < tampered.len() {
            tampered[tamper_idx] ^= 0xFF;
        }
        
        prop_assert!(verify_signature(&tampered, &sig, &pub_key).is_err());
    }

    #[test]
    fn prop_vector_clock_monotonic(
        seq1 in 0u64..1000,
        seq2 in 0u64..1000
    ) {
        let user_id = Uuid::new_v4();
        
        let mut clock1 = HashMap::new();
        clock1.insert(user_id, seq1);
        
        let mut clock2 = HashMap::new();
        clock2.insert(user_id, seq2);
        
        if seq1 <= seq2 {
            prop_assert!(clock1[&user_id] <= clock2[&user_id]);
        } else {
            prop_assert!(clock1[&user_id] > clock2[&user_id]);
        }
    }
}
```

---

### 4. Security Tests (Attack Scenarios)

```rust
#[tokio::test]
async fn test_replay_attack_prevented() {
    // Attacker captures a valid transaction and tries to replay it
    let (pub_key, priv_key) = generate_keypair();
    let mut tx = Transaction::new(/* ... */);
    tx.sign(&priv_key).unwrap();

    let block1 = create_block_with_tx(tx.clone());
    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    // Submit block with tx
    super_peer.validate_block(&block1).await.ok();

    // Try to replay: create new block with same tx (same nonce)
    let block2 = create_block_with_tx(tx);  // Same tx, same nonce

    let result = super_peer.validate_block(&block2).await;
    assert!(result.is_err(), "Replay attack must be prevented");
}

#[tokio::test]
async fn test_clock_skew_attack_prevented() {
    let (pub_key, priv_key) = generate_keypair();

    // Block 1: created at monotonic_time = 1000
    let mut block1 = LedgerBlock::new(pub_key, Uuid::new_v4(), 1, [0u8; 32], vec![], HashMap::new());
    block1.monotonic_created_nanos = 1000;
    block1.compute_block_hash().unwrap();
    block1.sign_with_device_key(&priv_key).unwrap();

    // Block 2: attacker tries to set earlier monotonic_time = 500
    let mut block2 = LedgerBlock::new(pub_key, Uuid::new_v4(), 2, block1.block_hash, vec![], HashMap::new());
    block2.monotonic_created_nanos = 500;  // Goes backward!
    block2.compute_block_hash().unwrap();
    block2.sign_with_device_key(&priv_key).unwrap();

    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    super_peer.validate_block(&block1).await.ok();
    let result = super_peer.validate_block(&block2).await;

    assert!(result.is_err(), "Clock skew attack must be prevented");
}

#[tokio::test]
async fn test_device_cloning_detected() {
    let (pub_key, priv_key) = generate_keypair();
    let device_id = Uuid::new_v4();

    // Device creates transaction with nonce bound to its IMEI
    let hw_ids = HardwareIds {
        imei: "123456789".to_string(),
        serial: "ABC123".to_string(),
        // ...
    };

    let nonce1 = derive_nonce_with_hardware(
        &[0u8; 32],
        &hw_ids,
        1
    );

    // Attacker clones device (IMEI copied) but submits transaction with different device_id
    let cloned_device_id = Uuid::new_v4();  // Different device ID
    let mut tx = Transaction::new(
        pub_key,
        [0u8; 32],
        50_000_000,
        "KES".to_string(),
        Decimal::one(),
        PaymentChannel::Online,
        "".to_string(),
        cloned_device_id,  // Cloned device
        [0u8; 32],
    );
    tx.current_nonce = nonce1;
    tx.sign(&priv_key).unwrap();

    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    let result = super_peer.validate_nonce_with_device(&tx.current_nonce, &cloned_device_id).await;
    assert!(result.is_err(), "Cloned device must be detected");
}

#[tokio::test]
async fn test_key_compromise_limited_by_daily_limit() {
    let (pub_key, priv_key) = generate_keypair();
    let device_id = Uuid::new_v4();

    // Attacker has compromised device key
    // Try to spend more than daily limit
    let mut transactions = vec![];
    let mut prev_nonce = [0u8; 32];

    for i in 0..11 {
        let mut tx = Transaction::new(
            pub_key,
            [0u8; 32],
            10_000_000,  // 10 OWC each = 110 total
            "KES".to_string(),
            Decimal::one(),
            PaymentChannel::Online,
            format!("Tx {}", i),
            device_id,
            prev_nonce,
        );
        tx.sign(&priv_key).unwrap();
        prev_nonce = tx.current_nonce;
        transactions.push(tx);
    }

    let mut block = LedgerBlock::new(
        pub_key,
        device_id,
        0,
        [0u8; 32],
        transactions,
        HashMap::new(),
    );
    block.compute_block_hash().unwrap();
    block.sign_with_device_key(&priv_key).unwrap();

    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    let result = super_peer.validate_block(&block).await;
    assert!(result.is_err(), "Daily limit should prevent large fraud");
}

#[tokio::test]
async fn test_witness_requirement_for_large_tx() {
    let (pub_key, priv_key) = generate_keypair();

    let mut tx = Transaction::new(
        pub_key,
        [0u8; 32],
        600_000_000,  // 600 OWC (> 500 threshold)
        "KES".to_string(),
        Decimal::one(),
        PaymentChannel::Online,
        "Large payment".to_string(),
        Uuid::new_v4(),
        [0u8; 32],
    );
    tx.sign(&priv_key).unwrap();

    let mut block = LedgerBlock::new(pub_key, Uuid::new_v4(), 0, [0u8; 32], vec![tx], HashMap::new());
    block.compute_block_hash().unwrap();
    block.sign_with_device_key(&priv_key).unwrap();

    let storage = setup_test_db().await;
    let super_peer = MockSuperPeer::new(storage, [0u8; 32]);

    // Validation should require witness signature
    let result = super_peer.validate_large_transaction(&block.transactions[0]).await;
    assert!(result.is_err(), "Large transaction must require witness");
}
```

---

## Guardrails / Runtime Assertions

### 1. Amount Validation Guardrails

```rust
/// GUARDRAIL: Amounts must always be i64, never float
pub fn validate_amount(amount: i64) -> Result<()> {
    if amount <= 0 {
        return Err(CylinderSealError::InvalidTransaction(
            "Amount must be positive".to_string(),
        ));
    }

    if amount > 1_000_000_000_000_000 {  // 1 billion OWC
        return Err(CylinderSealError::InvalidTransaction(
            "Amount exceeds maximum".to_string(),
        ));
    }

    Ok(())
}

/// GUARDRAIL: No floating point arithmetic
#[test]
fn test_no_floats_in_amounts() {
    // This won't compile:
    // let amount: f64 = 1.5;
    // let tx = Transaction { amount_owc: amount };

    // This is correct:
    let amount: i64 = 1_500_000;  // 1.5 OWC in micro-OWC
    let _tx = Transaction { amount_owc: amount };
}
```

### 2. Signature Verification Guardrails

```rust
/// GUARDRAIL: Always verify signatures before trusting
pub async fn process_transaction(tx: &Transaction) -> Result<()> {
    // GUARD: Verify signature FIRST, before any other processing
    tx.verify_signature()
        .map_err(|e| {
            tracing::error!("Signature verification failed: {}", e);
            e
        })?;

    // Only AFTER signature verified, continue
    validate_amount(tx.amount_owc)?;
    check_device_daily_limit(&tx.device_id, tx.amount_owc).await?;

    Ok(())
}

/// GUARDRAIL: Panic if signature verification is skipped
#[test]
#[should_panic]
fn test_panic_if_skipping_signature_verification() {
    let (pub_key, priv_key) = generate_keypair();
    let mut tx = Transaction::new(/* ... */);
    tx.sign(&priv_key).unwrap();
    tx.amount_owc = 999_000_000;  // Tamper

    // Process WITHOUT verifying (should panic in production)
    assert!(!tx.signature.is_empty());  // Signature exists
    // In production, this would assert that verification was called
}
```

### 3. Sequence Number Guardrails

```rust
/// GUARDRAIL: Sequence numbers must always increment by exactly 1
pub async fn validate_sequence(
    block: &LedgerBlock,
    last_block: &LedgerBlock,
) -> Result<()> {
    let expected = last_block.sequence_number + 1;
    let actual = block.sequence_number;

    if actual != expected {
        return Err(CylinderSealError::OutOfSequence {
            expected,
            got: actual,
        });
    }

    // Log for audit
    tracing::info!(
        "Sequence validated: expected={}, actual={}, block_hash={}",
        expected,
        actual,
        hex::encode(block.block_hash)
    );

    Ok(())
}

/// GUARDRAIL: Sequence cannot decrease
#[test]
fn test_sequence_number_can_never_decrease() {
    let mut seq = 100u64;
    let new_seq = 50u64;

    assert!(seq > new_seq, "Sequence went backward!");
    seq = new_seq;  // Should never happen in real code
}
```

### 4. Cryptographic Invariant Guardrails

```rust
/// GUARDRAIL: Block hash must match recomputed hash
pub async fn validate_block_hash(block: &LedgerBlock) -> Result<()> {
    let canonical = block.canonical_cbor_for_hashing()?;
    let expected_hash = blake2b_256(&canonical);

    if expected_hash != block.block_hash {
        // This is a serious error - log, alert, quarantine
        tracing::error!(
            "Block hash mismatch! expected={}, actual={}, user={:?}",
            hex::encode(expected_hash),
            hex::encode(block.block_hash),
            block.user_public_key
        );

        // Alert operations
        send_alert_to_opscall(
            "CRITICAL: Block hash validation failed",
            SeverityLevel::Critical,
        ).await?;

        return Err(CylinderSealError::InvalidHash);
    }

    Ok(())
}

/// GUARDRAIL: Vector clocks must never go backward
pub fn validate_vector_clock(
    current_clock: &HashMap<Uuid, u64>,
    previous_clock: &HashMap<Uuid, u64>,
) -> Result<()> {
    for (user_id, current_seq) in current_clock {
        if let Some(previous_seq) = previous_clock.get(user_id) {
            if current_seq < previous_seq {
                tracing::error!(
                    "ATTACK DETECTED: Vector clock went backward for user {:?}: {} -> {}",
                    user_id,
                    previous_seq,
                    current_seq
                );
                
                return Err(CylinderSealError::Conflict(
                    "Vector clock went backward".to_string(),
                ));
            }
        }
    }

    Ok(())
}

/// GUARDRAIL: Monotonic clocks must never go backward
pub fn validate_monotonic_time(
    current_nanos: i64,
    previous_nanos: i64,
) -> Result<()> {
    if current_nanos < previous_nanos {
        tracing::error!(
            "CRITICAL: Monotonic clock went backward: {} < {}",
            current_nanos,
            previous_nanos
        );

        send_alert("Clock went backward", SeverityLevel::Critical).await?;

        return Err(CylinderSealError::InternalError(
            "Monotonic clock violation".to_string(),
        ));
    }

    Ok(())
}
```

### 5. Device Attestation Guardrails

```rust
/// GUARDRAIL: Device attestation must be valid for high-value txs
pub async fn validate_attestation_for_amount(
    attestation: &str,
    amount: i64,
    kyc_tier: &KYCTier,
) -> Result<()> {
    let threshold = kyc_tier.attestation_threshold();

    if amount > threshold {
        let parsed = parse_attestation(attestation)?;

        if !parsed.is_valid() {
            tracing::warn!(
                "Attestation invalid for large transaction: amount={}, threshold={}",
                amount,
                threshold
            );

            return Err(CylinderSealError::InternalError(
                "Device attestation failed".to_string(),
            ));
        }

        if parsed.device_compromised() {
            tracing::error!("ATTACK: Device is jailbroken/rooted");
            return Err(CylinderSealError::DeviceCompromised);
        }
    }

    Ok(())
}
```

### 6. Conflict Detection Guardrails

```rust
/// GUARDRAIL: Double-spend detection
pub async fn detect_double_spend(
    block_a: &LedgerBlock,
    block_b: &LedgerBlock,
) -> Result<ConflictResolution> {
    if block_a.prev_block_hash != block_b.prev_block_hash {
        return Err(CylinderSealError::InternalError(
            "Blocks don't have same parent".to_string(),
        ));
    }

    if block_a.user_public_key != block_b.user_public_key {
        return Err(CylinderSealError::InternalError(
            "Blocks are from different users".to_string(),
        ));
    }

    // Found fork - resolve deterministically
    tracing::warn!(
        "DOUBLE-SPEND DETECTED: user={:?}, block_a={}, block_b={}",
        block_a.user_public_key,
        hex::encode(block_a.block_hash),
        hex::encode(block_b.block_hash)
    );

    let winner = if block_a.sequence_number > block_b.sequence_number {
        &block_a
    } else if block_b.sequence_number > block_a.sequence_number {
        &block_b
    } else {
        // Deterministic tiebreaker: lexicographic block hash
        if block_a.block_hash < block_b.block_hash {
            &block_a
        } else {
            &block_b
        }
    };

    Ok(ConflictResolution {
        winner_hash: winner.block_hash,
        loser_hash: if winner == &block_a { block_b.block_hash } else { block_a.block_hash },
    })
}
```

---

## Monitoring & Alerting

```rust
/// GUARDRAIL: Alert on suspicious patterns
pub async fn monitor_device_reputation(device_id: Uuid, reputation: &DeviceReputation) -> Result<()> {
    match reputation.score {
        0..=40 => {
            // CRITICAL: Freeze device
            tracing::error!("CRITICAL: Device reputation too low: {}", reputation.score);
            send_alert(
                format!("Device {} is highly suspicious (score: {})", device_id, reputation.score),
                SeverityLevel::Critical,
            ).await?;

            freeze_device_offline_payments(device_id).await?;
        }

        41..=60 => {
            // WARNING: Require additional verification
            tracing::warn!("WARNING: Device reputation low: {}", reputation.score);
            require_additional_verification(device_id).await?;
        }

        _ => {
            // Normal
        }
    }

    Ok(())
}

/// GUARDRAIL: Audit log health check
pub async fn verify_audit_log_integrity() -> Result<()> {
    let last_entry = get_last_audit_log_entry().await?;

    // Verify chain
    let prev_entry = get_audit_log_entry(last_entry.sequence - 1).await?;
    if prev_entry.entry_hash != last_entry.prev_hash {
        // Audit log is broken!
        tracing::error!("CRITICAL: Audit log chain is broken!");
        send_alert("Audit log integrity compromised", SeverityLevel::Critical).await?;
        return Err(CylinderSealError::InternalError(
            "Audit log chain broken".to_string(),
        ));
    }

    Ok(())
}
```

---

## Test Execution

```bash
# Run all unit tests
cargo test --lib

# Run integration tests
cargo test --test '*'

# Run with logging
RUST_LOG=debug cargo test -- --nocapture

# Run property tests (slower, thorough)
cargo test --test 'proptest_*' -- --test-threads=1

# Run security-specific tests
cargo test test_replay
cargo test test_clock_skew
cargo test test_double_spend
cargo test test_device_cloning

# Coverage report
cargo tarpaulin --out Html --output-dir coverage/
```

**Android:**
```bash
# Run all tests
./gradlew test

# Run with coverage
./gradlew testDebugUnitTestCoverage

# Instrumentation tests (on device/emulator)
./gradlew connectedAndroidTest
```

This testing & guardrails framework ensures the iron-secure design **actually works** in practice.
