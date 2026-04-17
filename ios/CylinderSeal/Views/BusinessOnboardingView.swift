import SwiftUI

struct BusinessOnboardingView: View {
    @EnvironmentObject private var container: AppContainer
    @StateObject private var vm: BusinessOnboardingViewModel
    let onDone: () -> Void

    init(onDone: @escaping () -> Void) {
        self.onDone = onDone
        // Placeholder; rebound in onAppear.
        let stub = AppContainer.bootstrap()
        _vm = StateObject(wrappedValue: BusinessOnboardingViewModel(
            wallet: stub.walletKeyManager,
            api: BusinessApiClient(baseURL: URL(string: "https://sp-baghdad.cbi.iq")!)
        ))
    }

    var body: some View {
        NavigationStack {
            Form {
                if vm.submitted {
                    Section {
                        Text("Submitted").font(.title2.bold())
                        if let msg = vm.statusMessage {
                            Text(msg)
                        }
                        Button("Done", action: onDone)
                    }
                } else {
                    Section("Business type") {
                        Picker("Type", selection: $vm.kind) {
                            ForEach(BusinessKind.allCases) { k in
                                VStack(alignment: .leading) {
                                    Text(k.label)
                                    Text(k.subtitle).font(.caption).foregroundStyle(.secondary)
                                }
                                .tag(k)
                            }
                        }
                        .pickerStyle(.inline)
                        .labelsHidden()
                    }
                    Section("Identity") {
                        TextField("Legal name", text: $vm.legalName)
                        TextField("Commercial registration (Sijel Tijari)", text: $vm.commercialRegistrationId)
                        TextField("Tax ID", text: $vm.taxId)
                        TextField("Industry code (ISIC, e.g. 4711)", text: $vm.industryCode)
                            .keyboardType(.numberPad)
                        TextField("Contact email", text: $vm.contactEmail)
                            .keyboardType(.emailAddress)
                            .textInputAutocapitalization(.never)
                        TextField("Registered address", text: $vm.registeredAddress, axis: .vertical)
                            .lineLimit(2...4)
                    }
                    if let err = vm.errorMessage {
                        Section { Text(err).foregroundStyle(.red) }
                    }
                    Section {
                        Button {
                            vm.submit()
                        } label: {
                            if vm.busy { ProgressView() }
                            else { Text("Submit registration").frame(maxWidth: .infinity) }
                        }
                        .disabled(vm.busy)
                    }
                    Section {
                        Text(
                            "CBI ops verifies your commercial registration and tax ID against the national registry before activating the account. This usually takes 1-3 business days."
                        )
                        .font(.caption)
                        .foregroundStyle(.secondary)
                    }
                }
            }
            .navigationTitle("Register business")
        }
    }
}
