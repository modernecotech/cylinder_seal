plugins {
    alias(libs.plugins.android.library)
    alias(libs.plugins.kotlin.android)
    alias(libs.plugins.hilt)
    kotlin("kapt")
}

android {
    namespace = "com.modernecotech.cylinderseal.core.cryptography"
    compileSdk = 34

    defaultConfig { minSdk = 24 }

    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_17
        targetCompatibility = JavaVersion.VERSION_17
    }
    kotlinOptions { jvmTarget = "17" }
}

dependencies {
    implementation(project(":core:core-common"))
    implementation(project(":core:core-ffi"))

    implementation(libs.kotlin.stdlib)
    implementation(libs.kotlinx.coroutines.core)
    implementation(libs.androidx.core.ktx)
    implementation(libs.tink.android)

    implementation(libs.hilt.android)
    kapt(libs.hilt.compiler)

    implementation(libs.timber)

    testImplementation(libs.junit)
}

kapt { correctErrorTypes = true }
