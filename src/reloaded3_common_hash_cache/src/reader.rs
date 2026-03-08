use bytemuck::{try_cast_slice, cast_slice};
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

        //Get a u64 array of relative path hashes for each file
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

    //TODO: replace cast_slice with try_cast_slice to prevent panics
    pub fn partial_hash(&self, entry: EntryIndex) -> u64 {cast_slice(&self.source.as_slice()[8..][0..self.header.number_of_entries() as usize * size_of::<u64>()])[entry.get()]}


    /// Gets the full hash for a file using an EntryIndex
    pub fn full_hash(&self, entry: EntryIndex) -> u64 {cast_slice(&self.source.as_slice()[8..][self.header.number_of_entries() as usize * size_of::<u64>()..self.header.number_of_entries() as usize * size_of::<u64>() * 2])[entry.get()]}

    /// Gets the path hash for a file using an EntryIndex
    pub fn path_hash(&self, entry: EntryIndex) -> u64 {cast_slice(&self.source.as_slice()[8..][self.header.number_of_entries() as usize * size_of::<u64>() * 2..self.header.number_of_entries() as usize * size_of::<u64>() * 3])[entry.get()]}

    /// Gets the last modified time for a file using an EntryIndex
    pub fn last_modified(&self, entry: EntryIndex) -> FILETIME {cast_slice(&self.source.as_slice()[8..][self.header.number_of_entries() as usize * size_of::<u64>() * 3..self.header.number_of_entries() as usize * size_of::<u64>() * 4])[entry.get()]}


}