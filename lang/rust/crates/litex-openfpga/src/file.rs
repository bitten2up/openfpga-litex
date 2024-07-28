use alloc::vec;
use alloc::{string::String, vec::Vec};
use litex_pac as pac;

pub struct File;

pub enum OpenFileMode {
    /// Read the file at the path, if any
    Read,
    /// Create the file if it doesn't exist
    CreateOnly,
    /// Resize/truncate the file if it exists
    ResizeOnly { expected_size: usize },
    /// Create the file if it doesn't exist, and resize/truncate it if it does
    CreateAndResize { expected_size: usize },
}

#[derive(Debug)]
pub enum OpenFileResult {
    Success,
    CreatedSuccess,
    SlotNotDefined,
    FileNotFound,
    MalformedPath,
    UnknownError,
}

#[derive(Debug)]
pub enum GetFileResult {
    SlotNotDefined,
    Utf8ParseError,
}

impl File {
    ///
    /// Reads the current file in the selected slot into the provided slice
    ///
    pub fn request_read(
        storage: &mut [u8],
        data_offset: usize,
        read_length: usize,
        bridge_slot_id: usize,
    ) {
        unsafe {
            let peripherals = pac::Peripherals::steal();

            if storage.len() < read_length {
                panic!("Attempted to overflow file read");
            }

            peripherals
                .APF_BRIDGE
                .data_offset
                .write(|w| w.bits(data_offset as u32));

            peripherals
                .APF_BRIDGE
                .transfer_length
                .write(|w| w.bits(read_length as u32));
            peripherals
                .APF_BRIDGE
                .ram_data_address
                .write(|w| w.bits(storage.as_mut_ptr() as u32));
            peripherals
                .APF_BRIDGE
                .slot_id
                .write(|w| w.bits(bridge_slot_id as u32));

            peripherals.APF_BRIDGE.request_read.write(|w| w.bits(1));
        };
    }

    ///
    /// Writes data from the start of the provided slice to the end of the provided length, with the file position offset by `data_offset`.
    ///
    pub fn request_write(
        storage: &[u8],
        data_offset: usize,
        write_length: usize,
        bridge_slot_id: usize,
        file_size: Option<usize>,
    ) {
        let peripherals = unsafe { pac::Peripherals::steal() };

        if storage.len() < write_length {
            panic!("Attempted to overflow file write");
        }

        peripherals
            .APF_BRIDGE
            .slot_id
            .write(|w| unsafe { w.bits(bridge_slot_id as u32) });

        peripherals
            .APF_BRIDGE
            .data_offset
            .write(|w| unsafe { w.bits(data_offset as u32) });

        if let Some(file_size) = file_size {
            peripherals
                .APF_BRIDGE
                .file_size
                .write(|w| unsafe { w.bits(file_size as u32) });
        }

        peripherals
            .APF_BRIDGE
            .transfer_length
            .write(|w| unsafe { w.bits(write_length as u32) });

        peripherals
            .APF_BRIDGE
            .ram_data_address
            .write(|w| unsafe { w.bits(storage.as_ptr() as u32) });

        peripherals
            .APF_BRIDGE
            .request_write
            .write(|w| unsafe { w.bits(1) });
    }

    ///
    /// Opens a file at the provided path, optionally creating a file at that path if it doesn't exist.
    ///
    /// Files can only be read or created in paths of the form `/[Assets | Saves]/[registered platform ID]/[common | registered core name]/*`.
    ///
    pub fn request_openfile(
        path: &str,
        mode: OpenFileMode,
        bridge_slot_id: usize,
    ) -> OpenFileResult {
        let peripherals = unsafe { pac::Peripherals::steal() };

        let expected_size = match mode {
            OpenFileMode::ResizeOnly { expected_size }
            | OpenFileMode::CreateAndResize { expected_size } => expected_size,
            _ => 0,
        };

        let mut path_storage = vec![0; 264];

        unsafe {
            let bytes = path.as_bytes();
            path_storage[0..bytes.len()].copy_from_slice(bytes);

            // Space is already filled with 0s from the Vec macro

            // Big endian
            path_storage[0x100] = 0;
            path_storage[0x101] = 0;
            path_storage[0x102] = 0;
            // Create file if doesn't exist
            path_storage[0x103] = match mode {
                OpenFileMode::Read => 0,
                OpenFileMode::CreateOnly => 1,
                OpenFileMode::ResizeOnly { .. } => 2,
                OpenFileMode::CreateAndResize { .. } => 3,
            };

            // Desired file size
            path_storage[0x104] = (expected_size >> 24) as u8;
            path_storage[0x105] = (expected_size >> 16) as u8;
            path_storage[0x106] = (expected_size >> 8) as u8;
            path_storage[0x107] = expected_size as u8;
        }

        peripherals
            .APF_BRIDGE
            .ram_data_address
            .write(|w| unsafe { w.bits(path_storage.as_ptr() as u32) });

        peripherals
            .APF_BRIDGE
            .slot_id
            .write(|w| unsafe { w.bits(bridge_slot_id as u32) });

        peripherals
            .APF_BRIDGE
            .request_openfile
            .write(|w| unsafe { w.bits(1) });

        File::block_op_complete();

        let result = peripherals.APF_BRIDGE.command_result_code.read().bits();

        match result {
            0 => OpenFileResult::Success,
            1 => OpenFileResult::CreatedSuccess,
            2 => OpenFileResult::SlotNotDefined,
            3 => OpenFileResult::FileNotFound,
            4 => OpenFileResult::MalformedPath,
            _ => OpenFileResult::UnknownError,
        }
    }

    ///
    /// Gets the current file path of the provided slot
    ///
    pub fn request_getfile(bridge_slot_id: usize) -> Result<String, GetFileResult> {
        let mut path_storage = vec![0; 256];

        let peripherals = unsafe { litex_pac::Peripherals::steal() };

        unsafe {
            peripherals
                .APF_BRIDGE
                .ram_data_address
                .write(|w| w.bits(path_storage.as_mut_ptr() as u32));

            peripherals.APF_BRIDGE.slot_id.write(|w| w.bits(1));

            peripherals.APF_BRIDGE.request_getfile.write(|w| w.bits(1));
        }

        File::block_op_complete();

        let result = peripherals.APF_BRIDGE.command_result_code.read().bits();

        match result {
            0 => {
                let string = String::from_utf8(path_storage);

                if let Ok(string) = string {
                    Ok(string)
                } else {
                    Err(GetFileResult::Utf8ParseError)
                }
            }
            _ => Err(GetFileResult::SlotNotDefined),
        }
    }

    ///
    /// Returns the current file size of the file in the provided slot
    ///
    pub fn size(bridge_slot_id: usize) -> usize {
        unsafe {
            let peripherals = pac::Peripherals::steal();

            peripherals
                .APF_BRIDGE
                .slot_id
                .write(|w| w.bits(bridge_slot_id as u32));

            // Ensure slot change and size read has occured, as it takes several cycles
            peripherals.APF_BRIDGE.slot_id.read().bits();

            peripherals.APF_BRIDGE.file_size.read().bits() as usize
        }
    }

    ///
    /// Returns true when operation complete, false when operation ongoing
    ///
    pub fn check_op_complete() -> bool {
        unsafe {
            let peripherals = pac::Peripherals::steal();

            peripherals.APF_BRIDGE.status.read().bits() == 1
        }
    }

    pub fn block_op_complete() {
        while !File::check_op_complete() {
            // Loop
        }
    }
}
