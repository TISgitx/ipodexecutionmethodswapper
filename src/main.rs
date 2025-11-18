// Copyright © 2025 TIS
// Licensed under the Creative Commons Attribution-NonCommercial-ShareAlike 4.0 International License
// Full license text: https://creativecommons.org/licenses/by-nc-sa/4.0/legalcode or in LICENSE file
use std::env;
use std::fs;
use std::io::Read;
use std::time::Duration;

struct Config {
    fast: bool,
}

impl Config {
    fn sleep(&self, dur: Duration) {
        if !self.fast {
            std::thread::sleep(dur);
        }
    }
}

// Searches for pattern in data and replaces the last byte of the found sequence with new_last_byte.
// Returns Some(offset) where offset is the position of the replaced byte (index within data), or None if not found.
fn patch_pattern_in_vec(data: &mut [u8], pattern: &[u8], new_last_byte: u8) -> Option<usize> {
    if pattern.is_empty() || data.len() < pattern.len() {
        return None;
    }
    if let Some(pos) = data.windows(pattern.len()).position(|w| w == pattern) {
        let target_index = pos + pattern.len() - 1;
        data[target_index] = new_last_byte;
        Some(target_index)
    } else {
        None
    }
}

// function for confirmation
fn press_enter() {
    let _ = std::io::stdin().read(&mut [0u8]);
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let fast_mode = args.iter().any(|a| a == "--fast"); // If there is --fast, enable fast mode
    let config = Config { fast: fast_mode };
    if fast_mode {
    println!("Fast mode enabled");
    }
    
    println!(r#"██████ ██████ ██████ ██  ██  █████ 
  ██   ██  ██ ██     ██████ ██     
  ██   ██████ ██████ ██████  █████ 
  ██   ██     ██     ██  ██     ██ 
██████ ██     ██████ ██  ██  █████ "#);
    config.sleep(Duration::from_millis(300));
    println!("iPod Execution Method Swapper");
    config.sleep(Duration::from_millis(300));
    println!("Copyright ©TIS 2025");
    config.sleep(Duration::from_secs(1));
    println!("This tool allows you to convert an Firmware.MSE file for a new way of modifying the firmware (themes, execution of unsigned code, etc.)");
    config.sleep(Duration::from_millis(500));
    println!("This utility modifies the file to provide a new method, but it can also make your device not boot at all (get a softbrick). So far, no accidents have been reported, but they can happen at the most unexpected moment (I wrote this just to be on the safe side)");
    println!("Are you sure you want to continue?");
    config.sleep(Duration::from_secs(2));
    println!("Press enter to confirm your choice, otherwise press Ctrl+C");
    press_enter();
    config.sleep(Duration::from_secs(1));
    
    // Reading a file in mse_out
    let mut mse_out = fs::read("Firmware.MSE")?;

    // --- CHECK FOR NANO 6 IPSW ---
    let unsupported_pattern: &[u8] = b"87232.0"; // hex: 38 37 32 33 32 2E 30
    if mse_out.windows(unsupported_pattern.len()).any(|w| w == unsupported_pattern) {
        println!("Error: This file is for nano 6, ONLY NANO 7 IS SUPPORTED");
        return Ok(()); // just exiting
    }
    // -----------------------------------------------------
    
    // ================================
    // 1. Checking ksid/soso signatures
    // ================================
    let sig1 = &mse_out[0x5004..][..4];
    let sig2 = &mse_out[0x5194..][..4];

    let expected_a = b"ksid";
    let expected_b = b"soso";

    let normal_ok = sig1 == expected_a && sig2 == expected_b;
    let reversed_ok = sig1 == expected_b && sig2 == expected_a;

    if !normal_ok && !reversed_ok {
        println!("Error: Signature mismatch, file looks corrupted.");
        return Ok(());
    }

    // If reversed → you will need to set signatures as in a normal scenario
    let need_fix_signatures = reversed_ok;
    
    // =========================================================
    // 2. Patch check: 87402.0\x04 and 87402.0\x03
    // =========================================================
    if !need_fix_signatures {
    let pattern_unpatched = b"87402.0\x04";
    let pattern_patched   = b"87402.0\x03";

    let count_patched = mse_out
        .windows(pattern_patched.len())
        .filter(|w| *w == pattern_patched)
        .count();

    let has_unpatched = mse_out
        .windows(pattern_unpatched.len())
        .any(|w| w == pattern_unpatched);

    // ---- Logics ----
    if has_unpatched {
        println!("The file is not fully patched. There may have been an attempt to modify this file before. I can apply an additional patch, but I do not guarantee that the device will boot after this.");
        config.sleep(Duration::from_secs(2));
        println!("Press Enter to continue, otherwise press Ctrl+C to cancel the operation");
        press_enter();
        config.sleep(Duration::from_secs(1));
        // Let's move on to patching below
    } else if count_patched == 11 {
        // Everything is already done
        println!("The file is fully patched, no changes required.");
        return Ok(());
    } else {
        println!("Error: file is corrupted (unclear patch state).");
        return Ok(());
    }
    }

    // =================================================================
    // 3. apply changes (put ksid/soso if necessary)
    // =================================================================
    if need_fix_signatures {
    mse_out[0x5004..][..4].copy_from_slice(b"ksid");
    mse_out[0x5194..][..4].copy_from_slice(b"soso");
    println!("Signatures fixed");
    }
    
    // =================================================================
    // 4. patching a specific pattern
    // =================================================================
    let pattern: &[u8] = b"87402.0\x04"; // search for \x04 → replace with \x03
    if let Some(idx) = patch_pattern_in_vec(&mut mse_out, pattern, 0x03) {
        println!("Patched Firmware.MSE at offset 0x{:X}", idx);
    } else {
        println!("Pattern not found in Firmware.MSE!");
    }

    fs::write("Firmware_modified.MSE", &mse_out)?;
    println!("Done.");

    Ok(())
}