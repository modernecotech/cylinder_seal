plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.protobuf)
}

android {
    namespace = "com.modernecotech.cylinderseal.core.network"
    compileSdk = 34

    defaultConfig {
        minSdk = 24
    }

    buildTypes {
        release {
            isMinifyEnabled = true
        }
    }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }

    kotlinOptions {
        jvmTarget = "17"
    }
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:3.24.0"
    }

    // This adds protobuf-kotlin code generation
    plugins {
        id("kotlin") {
            artifact = "io.grpc:protoc-gen-grpc-kotlin:1.4.0:osx-x86_64"
        }
        id("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:1.60.0:osx-x86_64"
        }
    }

    generateProtoTasks {
        all().forEach { task ->
            task.plugins {
                id("kotlin")
                id("grpc")
            }
            task.builtins {
                id("kotlin")
            }
        }
    }
}

dependencies {
    implementation(libs.kotlin.stdlib)
    implementation(libs.androidx.core.ktx)

    // gRPC and Protocol Buffers
    implementation(libs.grpc.stub)
    implementation(libs.grpc.protobuf.lite)
    implementation(libs.grpc.kotlin.stub)
    implementation(libs.grpc.android)
    implementation(libs.protobuf.java.lite)

    // Network
    implementation(libs.retrofit)
    implementation(libs.okhttp)
    implementation(libs.okhttp.logging)
    implementation(libs.conscrypt)

    // Serialization
    implementation(libs.kotlinx.serialization.json)

    // Testing
    testImplementation(libs.junit)
}
