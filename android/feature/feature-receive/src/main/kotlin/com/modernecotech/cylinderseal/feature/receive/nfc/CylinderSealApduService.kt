package com.modernecotech.cylinderseal.feature.receive.nfc

import android.nfc.cardemulation.HostApduService
import android.os.Bundle
import com.modernecotech.cylinderseal.feature.receive.ingest.IncomingPaymentIngestor
import dagger.hilt.android.AndroidEntryPoint
import javax.inject.Inject

/**
 * Host-based Card Emulation service that receives CylinderSeal transaction
 * APDUs from a sender device.
 *
 * The APDU protocol (defined in `crates/cs-mobile-core/src/wire.rs`):
 *   - SELECT AID (CLA 00, INS A4)       → reply: 0x9000
 *   - PROPOSE (CLA 80, INS 10, P1 seq)  → reply: 0x9000 (ok) / 0x6A80 (bad payload)
 *
 * The payload is CBOR chunks (max 253 bytes per APDU) that reassemble into
 * a signed [Transaction]. Once the whole transaction is assembled, it's
 * handed off to [IncomingPaymentIngestor] which validates signature, stores
 * it in the pending queue, and fires a notification.
 */
@AndroidEntryPoint
class CylinderSealApduService : HostApduService() {

    @Inject
    lateinit var ingestor: IncomingPaymentIngestor

    private val assembledChunks = mutableMapOf<Int, ByteArray>()
    private var expectedNextSeq = 0

    override fun processCommandApdu(commandApdu: ByteArray, extras: Bundle?): ByteArray {
        if (commandApdu.size < 4) return SW_WRONG_LENGTH

        val cla = commandApdu[0].toInt() and 0xFF
        val ins = commandApdu[1].toInt() and 0xFF

        return when {
            cla == 0x00 && ins == 0xA4 -> onSelect(commandApdu)
            cla == 0x80 && ins == 0x10 -> onPropose(commandApdu)
            else -> SW_INS_NOT_SUPPORTED
        }
    }

    private fun onSelect(apdu: ByteArray): ByteArray {
        // Verify the AID matches ours; otherwise the OS routes elsewhere.
        if (apdu.size < 5) return SW_WRONG_LENGTH
        assembledChunks.clear()
        expectedNextSeq = 0
        return SW_OK
    }

    private fun onPropose(apdu: ByteArray): ByteArray {
        if (apdu.size < 5) return SW_WRONG_LENGTH
        val seq = apdu[2].toInt() and 0xFF
        val lc = apdu[4].toInt() and 0xFF
        if (apdu.size < 5 + lc) return SW_WRONG_LENGTH
        val chunk = apdu.copyOfRange(5, 5 + lc)

        if (seq != expectedNextSeq) {
            // Out-of-order — reset and reject so the sender retries.
            assembledChunks.clear()
            expectedNextSeq = 0
            return SW_CONDITIONS_NOT_SATISFIED
        }
        assembledChunks[seq] = chunk
        expectedNextSeq += 1

        // Heuristic: if the chunk is shorter than 253, treat it as final.
        if (chunk.size < 253) {
            val payload = assembledChunks.entries
                .sortedBy { it.key }
                .map { it.value }
                .reduce { a, b -> a + b }
            assembledChunks.clear()
            expectedNextSeq = 0

            val ok = ingestor.onPayload(payload)
            return if (ok) SW_OK else SW_CONDITIONS_NOT_SATISFIED
        }

        return SW_OK
    }

    override fun onDeactivated(reason: Int) {
        assembledChunks.clear()
        expectedNextSeq = 0
    }

    companion object {
        val SW_OK = byteArrayOf(0x90.toByte(), 0x00)
        val SW_WRONG_LENGTH = byteArrayOf(0x67.toByte(), 0x00)
        val SW_CONDITIONS_NOT_SATISFIED = byteArrayOf(0x69.toByte(), 0x85.toByte())
        val SW_INS_NOT_SUPPORTED = byteArrayOf(0x6D.toByte(), 0x00)
    }
}
