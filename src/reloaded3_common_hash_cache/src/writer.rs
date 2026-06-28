use std::vec::Vec;
use crate::reader::{HashCacheReader, Source};
use crate::types::{FileInfo, HeaderV1, HeaderV1Builder};

pub trait WriteDestinationFactory: Send + Sync {
    type Error;
    type Destination: WriteDestination<Error = Self::Error>;

    /// Creates a destination with the specified capacity
    fn create_destination(&self, capacity: usize) -> Result<Self::Destination, Self::Error>;
}

pub trait WriteDestination: Send + Sync {
    type Error;
    type Reader: Source;

    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
    fn finish(self) -> Result<Self::Reader, Self::Error>;
}

pub struct HashCacheWriter {
    files: Vec<FileInfo>,
    version: u8,
    flag_a: bool,
    flag_b: bool,
    flag_c: bool,
    flag_d: bool,
    flag_e: bool
}

impl HashCacheWriter {
    /// Creates a new HashCacheWriter instance
    pub fn new(version: u8, flag_a: bool, flag_b: bool, flag_c: bool, flag_d: bool, flag_e: bool) -> Self {
        HashCacheWriter {
            files: Vec::new(),
            version,
            flag_a,
            flag_b,
            flag_c,
            flag_d,
            flag_e
        }
    }

    /// Adds file information to the hash cache
    pub fn add_file(&mut self, file_info: FileInfo){
        self.files.push(file_info);
    }

    /// Finalizes the writing process and returns a reader
    ///
    /// This method computes the required capacity for the destination,
    /// creates the destination using the factory, writes the data,
    /// and returns a reader for the written data.
    pub fn finalize<F: WriteDestinationFactory>(
        self,
        factory: F
    ) -> Result<HashCacheReader<<F::Destination as WriteDestination>::Reader>, F::Error> {
        let capacity:usize = (self.files.len() * 32) + size_of::<HeaderV1>();

        let mut destination = factory.create_destination(capacity)?;
        if self.version == 0 {
            let header = HeaderV1Builder::new()
                .with_version(self.version)
                .with_flag_a(self.flag_a)
                .with_flag_b(self.flag_b)
                .with_flag_c(self.flag_c)
                .with_flag_d(self.flag_d)
                .with_flag_e(self.flag_e)
                .with_number_of_entries(self.files.len() as u32)
                .with_padding(0)
                .build();
            destination.write(&header.into_bits().to_le_bytes())?;
        }
        else{
            panic!("HEADER VERSION NOT HANDLED");
        }

        for file in &self.files {
            destination.write(&file.partial_hash.to_le_bytes())?;
        }
        for file in &self.files {
            destination.write(&file.full_hash.to_le_bytes())?;
        }
        for file in &self.files {
            destination.write(&file.path_hash.to_le_bytes())?;
        }
        for file in &self.files {
            destination.write(&file.modify_time.to_le_bytes())?;
        }
        if self.flag_a {
            //TODO
        }
        let source = destination.finish()?;
        Ok(HashCacheReader::new(source).expect("Hash Cache likely corrupted, writer is likely broken!"))
    }
}