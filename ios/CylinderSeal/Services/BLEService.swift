import CoreBluetooth
import Foundation

/// BLE GATT peripheral.
///
/// Advertises one service with one writable characteristic. Senders
/// (another phone, a POS terminal) connect and write signed-CBOR payload
/// chunks; a zero-length write terminates the transfer and the
/// reassembled payload flows into [`IncomingPaymentIngestor`].
///
/// UUIDs match `android/feature/feature-receive/nfc` service definition
/// and the Linux POS — so all three ends interoperate.
@MainActor
final class BLEService: NSObject, ObservableObject {
    static let serviceUUID = CBUUID(nsuuid: UUID(uuidString: "7CB25EAA-CEA1-4E60-9B6D-70E15EA10001")!)
    static let characteristicUUID = CBUUID(nsuuid: UUID(uuidString: "7CB25EAA-CEA1-4E60-9B6D-70E15EA10002")!)

    @Published private(set) var isAdvertising = false
    @Published private(set) var isAvailable = false

    private var peripheral: CBPeripheralManager?
    private var characteristic: CBMutableCharacteristic?
    private var buffer = Data()

    private let ingestor: IncomingPaymentIngestor

    init(ingestor: IncomingPaymentIngestor) {
        self.ingestor = ingestor
        super.init()
    }

    func startAdvertising() {
        let peripheral = CBPeripheralManager(delegate: self, queue: .main)
        self.peripheral = peripheral
        // The rest happens in `peripheralManagerDidUpdateState` once the
        // radio is powered on.
    }

    func stopAdvertising() {
        peripheral?.stopAdvertising()
        peripheral?.removeAllServices()
        isAdvertising = false
    }

    private func publishService() {
        guard let peripheral else { return }
        let char = CBMutableCharacteristic(
            type: Self.characteristicUUID,
            properties: [.writeWithoutResponse, .write],
            value: nil,
            permissions: [.writeable]
        )
        let service = CBMutableService(type: Self.serviceUUID, primary: true)
        service.characteristics = [char]
        peripheral.add(service)
        self.characteristic = char

        let advert: [String: Any] = [
            CBAdvertisementDataServiceUUIDsKey: [Self.serviceUUID],
            CBAdvertisementDataLocalNameKey: "CylinderSeal"
        ]
        peripheral.startAdvertising(advert)
        isAdvertising = true
    }
}

extension BLEService: CBPeripheralManagerDelegate {
    nonisolated func peripheralManagerDidUpdateState(_ peripheral: CBPeripheralManager) {
        Task { @MainActor in
            self.isAvailable = peripheral.state == .poweredOn
            if peripheral.state == .poweredOn {
                self.publishService()
            }
        }
    }

    nonisolated func peripheralManager(
        _ peripheral: CBPeripheralManager,
        didReceiveWrite requests: [CBATTRequest]
    ) {
        Task { @MainActor in
            for req in requests {
                guard let value = req.value else {
                    peripheral.respond(to: req, withResult: .invalidAttributeValueLength)
                    continue
                }
                if value.isEmpty {
                    // End-of-stream sentinel — ingest reassembled payload.
                    let payload = self.buffer
                    self.buffer.removeAll(keepingCapacity: false)
                    if !payload.isEmpty {
                        self.ingestor.onPayload(payload, transport: .ble)
                    }
                } else {
                    self.buffer.append(value)
                }
                peripheral.respond(to: req, withResult: .success)
            }
        }
    }
}
