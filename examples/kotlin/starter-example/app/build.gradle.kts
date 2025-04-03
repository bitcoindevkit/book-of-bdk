plugins {
    id("org.jetbrains.kotlin.jvm") version "2.1.0"
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.bitcoindevkit:bdk-jvm:1.2.0")
}

java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(21)
    }
}

application {
    // Define the main class for the application.
    mainClass = "org.starterexample.AppKt"
}
