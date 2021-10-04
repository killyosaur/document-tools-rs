let header_signature: [u8; 8] = [0xd0, 0xcf, 0x11, 0xe0, 0xa1, 0xb1, 0x1a, 0xe1];
let header_clsid: [u8; 16] = [0x00; 16];
let minor_version: [u8; 2] = [0x00, 0x3e];
let major_version_3: [u8; 2] = [0x00, 0x03];
let major_version_4: [u8; 2] = [0x00, 0x04];
let byte_order: [u8; 2] = [0xff, 0xfe];
let sector_shift_3: [u8; 2] = [0x00, 0x09];
let sector_shift_4: [u8; 2] = [0x00, 0x0c];
let mini_sector_shift: [u8; 2] = [0x00, 0x06];
let reserved: [u8; 6] = [0x00; 6];
// number of directory sectors = 4 bytes
// version 3 of structured storage does not support 4 byte Number of Directory Sectors, set this value to 0x00000000
// number of fat sectors = 4 bytes
// first directory sector location = 4 bytes (location of directory stream)
// transaction signature number = 4 bytes (may be incremented on save)
let mini_stream_cutoff_size: [u8; 4] = [0x00, 0x00, 0x10, 0x00];
// first mini fat sector location = 4 bytes
// number of mini fat sectors = 4 bytes
// first DIFAT sector location = 4 bytes
// number of DIFAT sectors = 4 bytes
// DIFAT = 436 bytes (32 bit integer array containing first 109 FACT sector locations)

// Version 4 header is 512 bytes with a sector size of 4096. Remaining part of header are all zeroes (3584 bytes)

