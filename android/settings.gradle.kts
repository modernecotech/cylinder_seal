pluginManagement {
    repositories {
        google()
        mavenCentral()
        gradlePluginPortal()
    }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
        maven {
            url = uri("https://jitpack.io")
        }
    }
}

rootProject.name = "CylinderSeal"

// App
include(":app")

// Core modules
include(":core:core-common")
include(":core:core-crypto")
include(":core:core-network")
include(":core:core-database")
include(":core:core-datastore")
include(":core:core-model")

// Feature modules
include(":feature:feature-wallet")
include(":feature:feature-pay")
include(":feature:feature-receive")
include(":feature:feature-history")
include(":feature:feature-sync")
include(":feature:feature-onboarding")
include(":feature:feature-settings")

// Build logic (convention plugins)
include(":build-logic")
