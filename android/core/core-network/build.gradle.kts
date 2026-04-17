import com.google.protobuf.gradle.id

plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.hilt)
    alias(libs.plugins.protobuf)
    kotlin("kapt")
}

android {
    namespace = "com.modernecotech.cylinderseal.core.network"
    compileSdk = 34
    defaultConfig { minSdk = 24 }
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }
    // The proto file lives at workspace-root /proto/chain_sync.proto and
    // is symlinked to src/main/proto/chain_sync.proto so the protobuf
    // plugin picks it up via its default source-set lookup.
}

protobuf {
    protoc {
        artifact = "com.google.protobuf:protoc:${libs.versions.protobufVersion.get()}"
    }

    plugins {
        id("grpc") {
            artifact = "io.grpc:protoc-gen-grpc-java:${libs.versions.grpc.get()}"
        }
        id("grpckt") {
            artifact =
                "io.grpc:protoc-gen-grpc-kotlin:${libs.versions.grpcKotlin.get()}:jdk8@jar"
        }
    }

    generateProtoTasks {
        all().forEach { task ->
            task.plugins {
                id("grpc") { option("lite") }
                id("grpckt") { option("lite") }
            }
            task.builtins {
                id("java") { option("lite") }
                id("kotlin") { option("lite") }
            }
        }
    }
}

dependencies {
    implementation(project(":core:core-common"))
    implementation(project(":core:core-model"))
    implementation(project(":core:core-datastore"))

    implementation(libs.kotlin.stdlib)
    implementation(libs.kotlinx.coroutines.core)
    implementation(libs.androidx.core.ktx)

    // gRPC — exposed with `api` so consumer modules see the generated
    // proto classes without re-declaring these deps.
    api(libs.grpc.stub)
    api(libs.grpc.protobuf.lite)
    api(libs.grpc.kotlin.stub)
    api(libs.grpc.android)
    api(libs.grpc.okhttp)
    api(libs.protobuf.java.lite)
    api(libs.protobuf.kotlin.lite)

    implementation(libs.okhttp)
    implementation(libs.okhttp.logging)
    implementation(libs.conscrypt)

    implementation(libs.hilt.android)
    kapt(libs.hilt.compiler)

    implementation(libs.timber)
    testImplementation(libs.junit)
}

kapt { correctErrorTypes = true }
