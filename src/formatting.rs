//! Module for number formatting functions.
//!
//! This module contains utility functions for formatting numbers in various ways,
//! such as human-readable byte sizes.

use serde::Serializer;

/// Formats a byte size value into a human-readable string.
///
/// The function follows these rules:
/// - Uses units: B, KiB and MiB
/// - Switches from B to KiB at 1500 bytes
/// - Switches from KiB to MiB at 1500 * 1024 bytes
/// - Limits the number to a maximum of 4 characters by adjusting decimal places
///
/// # Arguments
///
/// * `bytes` - The size in bytes to format
///
/// # Returns
///
/// A formatted string representing the size with appropriate units
pub fn format_bytes(bytes: u32) -> String {
    const THRESHOLD: f64 = 1500.;
    const UNITS: &[&str] = &["B", "KiB", "MiB"];

    let mut value = bytes as f64;
    let mut unit_index = 0;

    // Keep dividing by 1024 until value is below threshold or we've reached the last unit
    while value >= THRESHOLD && unit_index < UNITS.len() - 1 {
        value /= 1024.0;
        unit_index += 1;
    }

    let unit = UNITS[unit_index];

    // Special case for bytes - no decimal places
    if unit_index == 0 {
        return format!("{bytes} {unit}");
    }

    // For KiB and MiB, format with appropriate decimal places

    // Determine number of decimal places to keep number under 4 chars
    if value < 10.0 {
        format!("{value:.2} {unit}") // e.g., 1.50 KiB, 9.99 MiB
    } else if value < 100.0 {
        format!("{value:.1} {unit}") // e.g., 10.5 KiB, 99.9 MiB
    } else {
        format!("{value:.0} {unit}") // e.g., 100 KiB, 999 MiB
    }
}

pub fn serialize_bytes<S: Serializer>(bytes: &u32, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format_bytes(*bytes))
}

/// Formats a number with "k" and "M" suffixes for thousands and millions.
///
/// The function follows these rules:
/// - Uses suffixes: none, k, and M
/// - Switches from no suffix to k at 1500
/// - Switches from k to M at 1500 * 1000
/// - Limits the number to a maximum of 4 characters by adjusting decimal places
///
/// # Arguments
///
/// * `number` - The number to format
///
/// # Returns
///
/// A formatted string representing the number with appropriate suffixes
pub fn format_number(number: u32) -> String {
    const THRESHOLD: f64 = 1500.;
    const UNITS: &[&str] = &["", "K", "M"];

    let mut value = number as f64;
    let mut unit_index = 0;

    // Keep dividing by 1000 until value is below threshold or we've reached the last unit
    while value >= THRESHOLD && unit_index < UNITS.len() - 1 {
        value /= 1000.0;
        unit_index += 1;
    }

    let unit = UNITS[unit_index];

    // Special case for numbers without suffix - no decimal places
    if unit_index == 0 {
        return format!("{number}");
    }

    // For k and M, format with appropriate decimal places

    // Determine number of decimal places to keep number under 4 chars
    if value < 10.0 {
        format!("{value:.1}{unit}")
    } else {
        format!("{value:.0}{unit}")
    }
}

pub fn serialize_number<S: Serializer>(number: &u32, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_str(&format_number(*number))
}

pub fn serialize_optional_number<S: Serializer>(
    opt_number: &Option<u32>,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    match opt_number {
        Some(number) => serializer.serialize_str(&format_number(*number)),
        None => serializer.serialize_none(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        // Test bytes format (below 1500 bytes)
        assert_eq!(format_bytes(0), "0 B");
        assert_eq!(format_bytes(1), "1 B");
        assert_eq!(format_bytes(1000), "1000 B");
        assert_eq!(format_bytes(1499), "1499 B");

        // Test kilobytes format (1500 bytes to 1500 * 1024 bytes)
        assert_eq!(format_bytes(1500), "1.46 KiB");
        assert_eq!(format_bytes(2048), "2.00 KiB");
        assert_eq!(format_bytes(5120), "5.00 KiB");
        assert_eq!(format_bytes(10240), "10.0 KiB");
        assert_eq!(format_bytes(51200), "50.0 KiB");
        assert_eq!(format_bytes(102400), "100 KiB");
        assert_eq!(format_bytes(512000), "500 KiB");
        assert_eq!(format_bytes(1048575), "1024 KiB");

        // Test megabytes format (above 1500 * 1024 bytes)
        assert_eq!(format_bytes(1536000), "1.46 MiB");
        assert_eq!(format_bytes(2097152), "2.00 MiB");
        assert_eq!(format_bytes(5242880), "5.00 MiB");
        assert_eq!(format_bytes(10485760), "10.0 MiB");
        assert_eq!(format_bytes(52428800), "50.0 MiB");
        assert_eq!(format_bytes(104857600), "100 MiB");
        assert_eq!(format_bytes(1073741824), "1024 MiB");
    }

    #[test]
    fn test_format_number() {
        // Test numbers without suffix (below 1500)
        assert_eq!(format_number(0), "0");
        assert_eq!(format_number(1), "1");
        assert_eq!(format_number(1000), "1000");
        assert_eq!(format_number(1499), "1499");

        // Test numbers with k suffix (1500 to 1500 * 1000)
        assert_eq!(format_number(1500), "1.5K");
        assert_eq!(format_number(2000), "2.0K");
        assert_eq!(format_number(5000), "5.0K");
        assert_eq!(format_number(10000), "10K");
        assert_eq!(format_number(50000), "50K");
        assert_eq!(format_number(100000), "100K");
        assert_eq!(format_number(500000), "500K");
        assert_eq!(format_number(999999), "1000K");

        // Test numbers with M suffix (above 1500 * 1000)
        assert_eq!(format_number(1500000), "1.5M");
        assert_eq!(format_number(2000000), "2.0M");
        assert_eq!(format_number(5000000), "5.0M");
        assert_eq!(format_number(10000000), "10M");
        assert_eq!(format_number(50000000), "50M");
        assert_eq!(format_number(100000000), "100M");
        assert_eq!(format_number(1000000000), "1000M");
    }
}
