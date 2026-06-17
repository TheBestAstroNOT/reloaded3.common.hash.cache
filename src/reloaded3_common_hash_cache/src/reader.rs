use bytemuck::try_cast_slice;
use crate::types::{HeaderV1, EntryIndex, TableEntry, ParseResult, FILETIME};
use hashbrown::HashTable;

pub trait Source: Send + Sync {
    fn as_slice(&self) -> &[u8];
}
pub struct HashCacheReader<S: Source>{
    source: S,
    header: HeaderV1,
    table: HashTable<TableEntry>
}

impl<S: Source> HashCacheReader<S> {
    fn get_section(&self, index: usize) -> Result<&[u64], ParseResult> {
        if index > 4 {
            return Err(ParseResult::IndexExceedsBounds);
        }
        let base = &self.source.as_slice()[8..];
        let number_of_entries = self.header.number_of_entries() as usize;
        let length = number_of_entries * size_of::<u64>();
        //We take section + 1 for zero indexing
        let start = length * index;
        let end = start + length;
        if base.len() < end {
            return Err(ParseResult::EOF);
        }
        try_cast_slice(&base[start..end]).map_err(|_| ParseResult::SliceConversionFailed)
    }

    /// Creates a new HashCacheReader instance from a source
    pub fn new(source: S) -> Result<Self, ParseResult> {
        let mut raw_bytes = source.as_slice();
        let raw_header = u64::from_le_bytes(raw_bytes[0..8].try_into().unwrap());

        //Remove the header from the slice to reduce the amount of addition we have to do later on
        raw_bytes = &raw_bytes[8..];

        //Get header from the read bits
        let header = HeaderV1::from_bits(raw_header);

        //Get the number of entries from the header
        let count = header.number_of_entries() as usize;

        //Safety check for EOF
        if raw_bytes.len() < count * size_of::<u64>() * 4 {
            return Err(ParseResult::EOF);
        }

        //Get an u64 array of relative path hashes for each file
        let path_hashes: &[u64] = match try_cast_slice(&raw_bytes[count * size_of::<u64>() * 2 .. count * size_of::<u64>() * 3]) {
            Ok(slice) => slice,
            Err(_) => return Err(ParseResult::SliceConversionFailed),
        };

        //Check if the flag for paths section is enabled or not
        if header.flag_A() {
            //TODO: IMPLEMENT PATHS SECTION
        }

        //Generate a hashtable that holds the index of an item in all arrays sorted by its relative path hash
        let mut table = HashTable::new();
        for (index, &hash) in path_hashes.iter().enumerate() {
            table.insert_unique(
                hash,
                TableEntry {
                    key: hash,
                    index: EntryIndex::new(index),
                    //TODO: IMPLEMENT PATHS SECTION INFORMATION
                    path_string_length: 0,
                    path_string_offset: 0,
                },
                |e: &TableEntry| e.key,
            );
        }
        Ok(HashCacheReader{
            source,
            header,
            table
        })
    }

    /// Returns the number of entries in the hash cache
    pub fn entry_count(&self) -> usize{
        self.header.number_of_entries() as usize
    }

    /// Checks if paths are included in this hash cache
    pub fn has_paths(&self) -> bool{
        self.header.flag_A()
    }

    /// Finds an entry by path hash and returns a wrapper around its index
    pub fn find_by_path_hash(&self, path_hash: u64) -> Option<EntryIndex> {self.table.find(path_hash, |entry| entry.key == path_hash).map(|e| e.index)}

    pub fn partial_hash(&self, entry: EntryIndex) -> Result<u64, ParseResult> {
        match self.get_section(0){
            Ok(section) => Ok(section[entry.get()]),
            Err(parse_result_propagate) => Err(parse_result_propagate),
        }
    }

    /// Gets the full hash for a file using an EntryIndex
    pub fn full_hash(&self, entry: EntryIndex) -> Result<u64, ParseResult> {
        match self.get_section(1){
            Ok(section) => Ok(section[entry.get()]),
            Err(parse_result_propagate) => Err(parse_result_propagate),
        }
    }

    /// Gets the path hash for a file using an EntryIndex
    pub fn path_hash(&self, entry: EntryIndex) -> Result<u64, ParseResult> {
        match self.get_section(2){
            Ok(section) => Ok(section[entry.get()]),
            Err(parse_result_propagate) => Err(parse_result_propagate),
        }
    }

    /// Gets the last modified time for a file using an EntryIndex
    pub fn last_modified(&self, entry: EntryIndex) -> Result<FILETIME, ParseResult> {
        match self.get_section(2){
            Ok(section) => Ok(section[entry.get()]),
            Err(parse_result_propagate) => Err(parse_result_propagate),
        }
    }
}