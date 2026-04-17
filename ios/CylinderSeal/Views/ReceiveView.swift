import CoreImage
import CoreImage.CIFilterBuiltins
import SwiftUI

struct ReceiveView: View {
    @EnvironmentObject private var container: AppContainer
    @StateObject private var vm: ReceiveViewModel

    init() {
        let empty = AppContainer.bootstrap()
        _vm = StateObject(
            wrappedValue: ReceiveViewModel(wallet: empty.walletKeyManager, ingestor: empty.ingestor)
        )
    }

    var body: some View {
        NavigationStack {
            VStack(spacing: 16) {
                Text("Show this code to the payer").font(.headline)
                if !vm.publicKeyHex.isEmpty, let cg = renderWalletQR(vm.publicKeyHex) {
                    Image(decorative: cg, scale: 1)
                        .interpolation(.none)
                        .resizable()
                        .aspectRatio(1, contentMode: .fit)
                        .frame(maxWidth: 300)
                    Text("PK \(String(vm.publicKeyHex.prefix(12)))…").font(.caption.monospaced())
                    Text("User \(String(vm.userId.prefix(8)))…").font(.caption.monospaced())
                }
                Button {
                    vm.showScanner = true
                } label: {
                    Label("Scan sender's QR", systemImage: "qrcode.viewfinder")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.borderedProminent)

                Button {
                    container.nfcReader.beginSession()
                } label: {
                    Label("Tap sender's phone", systemImage: "wave.3.right")
                        .frame(maxWidth: .infinity)
                }
                .buttonStyle(.bordered)
                .disabled(!container.nfcReader.isAvailable)
                Spacer()
            }
            .padding()
            .navigationTitle("Receive")
            .onAppear(perform: vm.onAppear)
            .sheet(isPresented: $vm.showScanner) {
                QRScannerView(onScan: vm.handleScan)
            }
        }
    }

    private func renderWalletQR(_ publicKeyHex: String) -> CGImage? {
        let filter = CIFilter.qrCodeGenerator()
        filter.message = Data("CS1:PK:\(publicKeyHex)".utf8)
        filter.correctionLevel = "M"
        guard let output = filter.outputImage else { return nil }
        let scaled = output.transformed(by: CGAffineTransform(scaleX: 8, y: 8))
        let context = CIContext()
        return context.createCGImage(scaled, from: scaled.extent)
    }
}
