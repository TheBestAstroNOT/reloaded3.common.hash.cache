use bitfields::bitfield;

const EPOCH_DIFFERENCE: i64 = 11_644_473_600;
const NANOS_PER_SEC: i64 = 1_000_000_000;
const WINDOWS_TICK: i64 = 100; // 100 nanoseconds;
pub type FILETIME = u64;

/// Converts a Unix timespec (seconds and nanoseconds) to a Windows timestamp
/// (100-nanosecond intervals since January 1, 1601 UTC)
///
/// This function handles both positive and negative timestamps, accounting for the difference
/// between Unix epoch (1970-01-01) and Windows epoch (1601-01-01).
///
/// Note: This function does not perform explicit overflow checks for performance reasons.
/// It should handle most practical timestamp values, but extreme values may cause overflow.
///
/// # Arguments
/// The unix timespec components.
///
/// * `seconds` - Signed seconds since Unix epoch (January 1, 1970)
/// * `nanoseconds` - Additional nanoseconds (always non-negative)
///
/// # Returns
///
/// The Windows timestamp as an i64 (FILETIME)
fn timespec_to_windows_timestamp(seconds: i64, nanoseconds: u32) -> i64 {
    // Convert seconds to Windows ticks
    let second_ticks = seconds * (NANOS_PER_SEC / WINDOWS_TICK);

    // Convert nanoseconds to Windows ticks
    let nano_ticks = (nanoseconds as i64) / WINDOWS_TICK;

    // Add epoch difference in ticks
    let epoch_diff_ticks = EPOCH_DIFFERENCE * (NANOS_PER_SEC / WINDOWS_TICK);

    // Combine all components
    second_ticks + nano_ticks + epoch_diff_ticks
}

pub struct FileInformation{
    pub partial_hash: u64,
    pub full_hash: u64,
    pub path_hash: u64,
    pub path: u16,
    pub modify_time: FILETIME
}

#[bitfield(u64)]
#[derive(Copy, Clone)]
pub struct HeaderV1{
    #[bits(3)]
    version:u8,
    flag_A:bool,
    flag_B:bool,
    flag_C:bool,
    flag_D:bool,
    flag_E:bool,
    #[bits(24)]
    number_of_entries: u32,
    #[bits(32)]
    padding: u32,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct EntryIndex(usize);
impl EntryIndex {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn get(self) -> usize {
        self.0
    }
}

pub struct TableEntry{
    pub key: u64,
    pub index: EntryIndex,
    pub path_string_offset: usize,
    pub path_string_length: usize,
}

pub enum ParseResult{
    EOF,
    SliceConversionFailed
}