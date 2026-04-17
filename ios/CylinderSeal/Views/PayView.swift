import CoreImage
import CoreImage.CIFilterBuiltins
import SwiftUI

struct PayView: View {
    @EnvironmentObject private var container: AppContainer
    @StateObject private var vm: PayViewModel

    init() {
        let empty = AppContainer.bootstrap() // replaced onAppear; see note in Onboarding
        _vm = StateObject(wrappedValue: PayViewModel(wallet: empty.walletKeyManager, database: empty.database))
    }

    var body: some View {
        NavigationStack {
            Form {
                Section("Recipient") {
                    TextField("Public key (hex)", text: $vm.recipientHex)
                        .textInputAutocapitalization(.never)
                        .disableAutocorrection(true)
                }
                Section("Amount") {
                    TextField("IQD", text: $vm.amountInput)
                        .keyboardType(.decimalPad)
                    TextField("Memo (optional)", text: $vm.memo)
                }
                Section("Transport") {
                    Picker("Channel", selection: $vm.channel) {
                        Text("NFC").tag(PaymentChannel.nfc)
                        Text("BLE").tag(PaymentChannel.ble)
                        Text("Online").tag(PaymentChannel.online)
                    }
                }
                if let err = vm.error {
                    Section { Text(err).foregroundStyle(.red) }
                }
                Section {
                    Button(action: vm.submit) {
                        if vm.busy { ProgressView() } else { Text("Sign & send").frame(maxWidth: .infinity) }
                    }
                    .buttonStyle(.borderedProminent)
                    .disabled(vm.busy)
                }
                if let qr = vm.qrPayload {
                    Section("Show this to the recipient") {
                        if let cg = renderQR(qr) {
                            Image(decorative: cg, scale: 1)
                                .interpolation(.none)
                                .resizable()
                                .aspectRatio(1, contentMode: .fit)
                        }
                        if let txId = vm.transactionId {
                            Text("Tx \(String(txId.prefix(12)))…").font(.caption.monospaced())
                        }
                        Button("Done", action: vm.reset)
                    }
                }
            }
            .navigationTitle("Send")
        }
    }

    private func renderQR(_ payload: String) -> CGImage? {
        let filter = CIFilter.qrCodeGenerator()
        filter.message = Data(payload.utf8)
        filter.correctionLevel = "M"
        guard let output = filter.outputImage else { return nil }
        let scaled = output.transformed(by: CGAffineTransform(scaleX: 8, y: 8))
        let context = CIContext()
        return context.createCGImage(scaled, from: scaled.extent)
    }
}
