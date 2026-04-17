package iq.cbi.cylinder_seal

import android.nfc.cardemulation.HostApduService
import android.os.Bundle

/**
 * HCE service that the OS routes to whenever a reader sends
 * `SELECT AID F0 CB CD 01 00`. The Dart side hands us the signed CBOR
 * via the `nfc_hce` MethodChannel; we then chunk it into the same
 * `propose` frames cs-bridge::build_nfc_apdus produces and reply one
 * chunk per `processCommandApdu` call.
 *
 * NB: we keep the chunked payload in a static field because Android
 * instantiates `HostApduService` itself (via the OS), so the activity
 * cannot pass an instance reference in. The activity's lifecycle
 * arms/disarms via the companion-object methods below.
 */
class CsHostApduService : HostApduService() {

    private var seq: Int = 0

    override fun processCommandApdu(cmd: ByteArray?, extras: Bundle?): ByteArray {
        val payload = currentPayload ?: return SW_NOT_READY
        if (cmd == null) return SW_BAD_COMMAND

        // SELECT AID — reply 9000 to advertise that we are the right app.
        if (cmd.size >= 5 && cmd[0] == 0x00.toByte() && cmd[1] == 0xA4.toByte()) {
            seq = 0
            return SW_OK
        }

        // Reader is asking for the next chunk.
        val chunkSize = 253
        val start = seq * chunkSize
        if (start >= payload.size) {
            disarm()
            return SW_OK
        }
        val end = minOf(start + chunkSize, payload.size)
        val chunk = payload.copyOfRange(start, end)
        seq += 1

        // Final chunk → append SW_OK; intermediate chunks → append SW_MORE_DATA.
        val sw = if (end == payload.size) SW_OK else SW_MORE_DATA
        return chunk + sw
    }

    override fun onDeactivated(reason: Int) {
        seq = 0
    }

    companion object {
        @JvmStatic
        @Volatile
        private var currentPayload: ByteArray? = null

        fun armPayload(cbor: ByteArray) {
            currentPayload = cbor
        }

        fun disarm() {
            currentPayload = null
        }

        private val SW_OK = byteArrayOf(0x90.toByte(), 0x00)
        private val SW_MORE_DATA = byteArrayOf(0x61.toByte(), 0x00)
        private val SW_NOT_READY = byteArrayOf(0x6F.toByte(), 0x00)
        private val SW_BAD_COMMAND = byteArrayOf(0x6D.toByte(), 0x00)
    }
}
