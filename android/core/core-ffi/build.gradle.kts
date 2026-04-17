plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
}

android {
    namespace = "com.modernecotech.cylinderseal.core.ffi"
    compileSdk = 34

    defaultConfig {
        minSdk = 24
        ndk {
            abiFilters += listOf("arm64-v8a", "armeabi-v7a", "x86_64", "x86")
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions { jvmTarget = "17" }

    sourceSets {
        getByName("main") {
            // UniFFI-generated Kotlin bindings go here after `cargo run --bin
            // uniffi-bindgen generate` is invoked against the UDL in
            // crates/cs-mobile-core. The native libraries are placed under
            // core-ffi/src/main/jniLibs/<abi>/libcs_mobile_core.so.
            java.srcDir("src/main/kotlin")
            jniLibs.srcDir("src/main/jniLibs")
        }
    }
}

dependencies {
    implementation(libs.kotlin.stdlib)
    implementation(libs.kotlinx.coroutines.core)
    implementation(libs.jna) {
        artifact {
            type = "aar"
        }
    }
    implementation(libs.timber)
    testImplementation(libs.junit)
}
