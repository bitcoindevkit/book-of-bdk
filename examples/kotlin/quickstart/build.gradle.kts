plugins {
    kotlin("jvm") version "1.8.0"
    application
}

repositories {
    mavenCentral()
}

dependencies {
    implementation("org.jetbrains.kotlin:kotlin-stdlib")
    // Add the Bitcoin Development Kit (BDK) dependency
    implementation("org.bitcoindevkit:bdk-jvm:0.28.0")
}

application {
    mainClass.set("MainKt")
}

kotlin {
    jvmToolchain(17) // This sets the JVM target to Java 17
}