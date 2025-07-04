# Create grouped Rust library subprojects for Rust Tutorial Basics 621

$structure = @{
    "Getting Started & Basics" = @(
        "Initializing a project locally",
        "Variables (number, strings and bools)",
        "Conditionals, loops",
        "Functions"
    )
    "Data Structures & Organization" = @(
        "Structs",
        "Enums",
        "Optional/Result"
    )
    "Control Flow & Data Interaction" = @(
        "Pattern matching",
        "Package management"
    )
    "Core Memory Safety & Performance Concepts" = @(
        "Memory management",
        "Mutability",
        "Stack vs heap",
        "Ownership",
        "Borrowing",
        "References"
    )
    "Advanced Language Features" = @(
        "Traits",
        "Generics",
        "Lifetimes"
    )
    "Concurrency & Advanced Abstractions" = @(
        "Multithreading",
        "Macros",
        "Futures/async await"
    )
}

foreach ($group in $structure.Keys) {
    foreach ($topic in $structure[$group]) {
        # Sanitize folder names for Rust package naming
        $projName = $topic.ToLower() -replace '[^a-z0-9]+', '_'
        $groupDir = $group -replace '[^a-zA-Z0-9]+', '_'
        $projPath = ".\$groupDir\$projName"
        $srcPath = "$projPath\src"
        New-Item -ItemType Directory -Path $srcPath -Force | Out-Null

        # Create minimal Cargo.toml for each subproject
        $cargoToml = @"
[package]
name = "$projName"
version = "0.1.0"
edition = "2021"

[dependencies]
"@
        Set-Content -Path "$projPath\Cargo.toml" -Value $cargoToml

        # Create minimal src/lib.rs
        $libRs = "// $topic library root"
        Set-Content -Path "$srcPath\lib.rs" -Value $libRs
    }
}

# Create top-level Cargo.toml for the workspace
$members = @()
foreach ($group in $structure.Keys) {
    foreach ($topic in $structure[$group]) {
        $projName = $topic.ToLower() -replace '[^a-z0-9]+', '_'
        $groupDir = $group -replace '[^a-zA-Z0-9]+', '_'
        $projPath = "$groupDir/$projName"
        $members += $projPath
    }
}
$workspaceToml = @"
[workspace]
members = [
$(($members | ForEach-Object { "    `"$($_)`"" }) -join ",`n")
]
"@
Set-Content -Path ".\Cargo.toml" -Value $workspaceToml