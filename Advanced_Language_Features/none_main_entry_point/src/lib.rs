// None_main_entry_point library root
// src/main.rs (or src/lib.rs if building a shared library)

// We need to tell the Rust compiler not to link the standard library
// and not to use the default main entry point
#![no_std]
#![no_main] // Only if you're truly replacing the standard 'main' entry point

// use core::panic::PanicInfo;

// This attribute tells the Rust compiler not to mangle the function name.
// This means the linker will look for a symbol named '_start' (or whatever you name it)
// directly, without Rust adding unique identifiers to it.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This is our custom entry point.
    // In a real bare-metal scenario, you'd initialize hardware here, etc.
    // For a simple example, we'll just loop indefinitely (common for embedded).
    loop {
        // Here you would put your actual program logic
        // For demonstration, let's try to print (requires specific setup for no_std)
        // Without special stdout setup for #![no_std], this won't actually print to console.
        // For a desktop example, you might link with a C main function.
    }
}

// In a #[no_std] context, you must define a panic handler
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {}
// }

// In Rust, the main function is almost always the default and primary entry point for a standalone executable program. When you compile a Rust program into an executable, the linker is configured to start execution at the main function.
// However, to answer your question directly: No, the main function is not always the only possible entry point. Other entry points are possible, primarily through the use of:
// #[no_mangle] and C/FFI (Foreign Function Interface): This is the most common way to create an alternative entry point, especially when interacting with code written in other languages (like C) or when building operating system kernels or embedded systems where a standard C runtime isn't present.
// Custom #[start] attribute (unstable/nightly Rust): This is a highly unstable feature in Rust's nightly builds that allows you to designate a function other than main as the program's entry point. It's generally not used for typical application development.
// No main function in libraries: It's important to distinguish between executables and libraries (crates). Libraries do not have a main function because they are not meant to be run directly; they provide functionality for other programs to use.
// Example 1: Alternative Entry Point for C Compatibility (#[no_mangle])
// This is common when you're building a Rust library that will be called from C code, or when you're writing a very low-level program like an OS kernel without a standard C runtime. The #[no_mangle] attribute tells the Rust compiler not to mangle the function name (i.e., keep its name exactly as written), which makes it callable by external linkers.
// Scenario: Imagine you're writing a simple Rust function that you want to be the entry point for a very minimal bare-metal application or a shared library loaded by a C program.
// Rust
// // src/main.rs (or src/lib.rs if building a shared library)
// // We need to tell the Rust compiler not to link the standard library
// // and not to use the default main entry point
// #![no_std]
// #![no_main] // Only if you're truly replacing the standard 'main' entry point
// use core::panic::PanicInfo;
// // This attribute tells the Rust compiler not to mangle the function name.
// // This means the linker will look for a symbol named '_start' (or whatever you name it)
// // directly, without Rust adding unique identifiers to it.
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
// This is our custom entry point.
// In a real bare-metal scenario, you'd initialize hardware here, etc.
// For a simple example, we'll just loop indefinitely (common for embedded).
// loop {
// Here you would put your actual program logic
// For demonstration, let's try to print (requires specific setup for no_std)
// Without special stdout setup for #![no_std], this won't actually print to console.
// For a desktop example, you might link with a C main function.
// }
// }
// // In a #[no_std] context, you must define a panic handler
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
// loop {}
// }
// Explanation:
// #![no_std]: This directive tells Rust not to link the standard library, which depends on an underlying operating system. This is crucial for bare-metal programming.
// #![no_main]: This tells Rust not to use the default main entry point provided by the standard library.
// #[no_mangle]: Ensures the function name _start remains _start in the compiled binary, making it findable by the linker as the program's entry point (which is often _start or main depending on the system and linker conventions).
// pub extern "C" fn _start() -> !:
// pub: Makes it public.
// extern "C": Specifies the C calling convention, which is standard for system-level entry points.
// _start: A common name for the actual entry point in many Unix-like systems, which then calls the C runtime's main. By providing our own, we bypass the C runtime.
// -> !: Indicates that this function never returns (e.g., it loops forever or halts the system), which is typical for low-level entry points.
// #[panic_handler]: Since we're not using the standard library, we must provide our own panic handler, as the default one from std won't be linked.