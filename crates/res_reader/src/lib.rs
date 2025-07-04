use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read, Seek, SeekFrom},
    path::{Path, PathBuf},
};

mod color_utils;

mod palette;

mod parse_big_image;
use parse_big_image::parse_big_image;

mod parse_simple_image;
use parse_simple_image::parse_simple_image;

pub struct ImageData {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

static MAX_IMAGE_WIDTH: i16 = 640;
static MAX_IMAGE_HEIGHT: i16 = 480;

pub struct TableOfContentsEntry {
    pub tag: [u8; 8],
    pub data_offset: i32,
    pub data_size: i32,
}

#[derive(Debug, Clone)]
pub struct EntryInfo {
    pub tag: String,
    pub start_offset: usize,
    pub byte_length: usize,
}

#[derive(Debug, Clone)]
pub struct ResReader {
    pub file_path: PathBuf,
    pub table_of_contents: HashMap<String, EntryInfo>,
}

impl ResReader {
    pub fn new(file_path: &Path) -> Self {
        let mut res_reader = Self {
            file_path: file_path.to_path_buf(),
            table_of_contents: HashMap::new(),
        };
        res_reader.read_toc().map_err(|e| {
            log::error!("Failed to read TOC from file: {}", file_path.display());
			log::error!("  - {}", e);
			panic!("Failed to read TOC: {}", e);
		}).unwrap();
        res_reader
    }

    pub fn read_file(&self, tag: &str) -> Option<Vec<u8>> {
        let entry = self.table_of_contents.get(tag)?;

        let res_file = File::open(&self.file_path).ok()?;
        let mut reader = BufReader::new(res_file);

        reader
            .seek(SeekFrom::Start(entry.start_offset as u64))
            .ok()?;

        let mut buffer = vec![0u8; entry.byte_length];
        reader.read_exact(&mut buffer).ok()?;

        Some(buffer)
    }

	pub fn read_image(&self, tag: &str) -> Option<ImageData> {
		if let Some(file_data) = self.read_file(tag) {
			if let Some(image_data) = parse_simple_image(&file_data) {
				return Some(image_data);
			}
			if let Some(image_data) = parse_big_image(&file_data) {
				return Some(image_data);
			}
		}
		None
	}

    fn read_toc(&mut self) -> Result<(), String> {
        let res_file = File::open(&self.file_path).map_err(|e| {
            log::error!("Failed to open resource file: {}", e);
            format!("Failed to open resource file: {}", e)
        })?;

        let mut reader = BufReader::new(res_file);

        // -- read the resource header --

        let mut magic_seq = [0u8; 4];

        reader
            .read_exact(&mut magic_seq)
            .expect("Failed to read magic sequence");

        // if magic_seq != [b'R', b'E', b'S', 0] || magic_seq != [0, 0, 0, 0] {
        //     log::error!("Invalid resource file format: incorrect magic sequence");
        //     panic!("Invalid resource file format: incorrect magic sequence");
        // }

        let mut offset_bytes = [0u8; 4];
        reader
            .read_exact(&mut offset_bytes)
            .expect("Failed to read offset");
        let offset = i32::from_le_bytes(offset_bytes);

        let mut size_bytes = [0u8; 4];
        reader
            .read_exact(&mut size_bytes)
            .expect("Failed to read size");
        let size = i32::from_le_bytes(size_bytes);

        // -- read the TOC entries --

        reader
            .seek(SeekFrom::Start(offset as u64))
            .expect("Failed to seek to TOC");

        for _ in 0..size / std::mem::size_of::<TableOfContentsEntry>() as i32 {
            let mut tag = [0u8; 8];
            reader.read_exact(&mut tag).expect("Failed to read tag");

            let mut data_offset_bytes = [0u8; 4];
            reader
                .read_exact(&mut data_offset_bytes)
                .expect("Failed to read data offset");
            let data_offset = i32::from_le_bytes(data_offset_bytes);

            let mut data_size_bytes = [0u8; 4];
            reader
                .read_exact(&mut data_size_bytes)
                .expect("Failed to read data size");
            let data_size = i32::from_le_bytes(data_size_bytes);

            self.table_of_contents.insert(
                String::from_utf8_lossy(&tag)
                    .trim_end_matches('\0')
                    .to_string(),
                EntryInfo {
                    tag: String::from_utf8_lossy(&tag)
                        .trim_end_matches('\0')
                        .to_string(),
                    start_offset: data_offset as usize,
                    byte_length: data_size as usize,
                },
            );
        }

        Ok(())
    }
}
