use std::io::{Read, Write};
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;

fn main() {
    println!("UART Host Verification Test");
    println!("==========================");

    // Build the test kernel first
    println!("Building test kernel...");
    let build_status = Command::new("cargo")
        .args(["build", "--release"])
        .current_dir("..")
        .status()
        .expect("Failed to build test kernel");

    if !build_status.success() {
        eprintln!("✗ Failed to build test kernel");
        std::process::exit(1);
    }

    println!("✓ Test kernel built successfully");

    // Run QEMU with UART test
    println!("\nRunning QEMU with UART test...");

    let mut child = Command::new("qemu-system-riscv32")
        .args([
            "-machine",
            "virt",
            "-nographic",
            "-serial",
            "stdio",
            "-monitor",
            "none",
            "-bios",
            "none",
            "-kernel",
            "../target/riscv32imac-unknown-none-elf/release/ns16550a-tests",
        ])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start QEMU");

    let mut stdin = child.stdin.take().expect("Failed to capture stdin");
    let mut stdout = child.stdout.take().expect("Failed to capture stdout");

    // Give QEMU time to start
    thread::sleep(Duration::from_millis(300));

    // Read initial output (PUT test)
    let mut buffer = [0u8; 1024];
    let bytes_read = stdout.read(&mut buffer).expect("Failed to read stdout");
    let output = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("QEMU Output:\n{}", output);

    // Verify expected patterns
    let expected_patterns = ["UART", "Hello, World!"];
    let mut all_passed = true;

    for pattern in expected_patterns {
        if output.contains(pattern) {
            println!("✓ Found expected pattern: {}", pattern);
        } else {
            println!("✗ Missing expected pattern: {}", pattern);
            all_passed = false;
        }
    }

    // Test GET operation - send data to QEMU while it's waiting
    println!("\nTesting UART GET operation...");
    let test_data = b"X";

    // Send input while QEMU is in waiting loop
    stdin
        .write_all(test_data)
        .expect("Failed to write to stdin");
    stdin.flush().expect("Failed to flush stdin");

    // Give time for processing
    thread::sleep(Duration::from_millis(500));

    // Read response (should contain echo and acknowledgment)
    let bytes_read = stdout.read(&mut buffer).unwrap_or(0);
    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("Response to input: {}", response);

    // Check if we see echo or acknowledgment
    if response.eq("X!") {
        println!("✓ UART GET operation working");
    } else {
        println!("✗ UART GET operation: no clear acknowledgment");
        all_passed = false;
    }

    // Wait for QEMU to exit with timeout
    let timeout = Duration::from_secs(1);
    let start = std::time::Instant::now();

    let status = loop {
        if let Some(status) = child.try_wait().expect("Failed to wait for QEMU") {
            break status;
        }

        if start.elapsed() >= timeout {
            println!("✗ QEMU did not exit within timeout period");
            child.kill().expect("Failed to kill QEMU process");
            std::process::exit(1);
        }

        thread::sleep(Duration::from_millis(100));
    };

    if all_passed && status.success() {
        println!("\n✓ All UART tests passed!");
        std::process::exit(0);
    } else {
        println!("\n✗ Some UART tests failed!");
        std::process::exit(1);
    }
}
