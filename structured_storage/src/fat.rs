let end_of_chain = 0xfffffffe;
let free_sect = 0xffffffff;
let fat_sect = 0xfffffffd;
let dif_sect = 0xfffffffc;

// adding 1 to the sector number due to the header being stored at 0
macro_rules! sector_number_to_byte_offset {
  ($sector_num:u8, $sector_size:u8) => {
    ($sector_num + 1) * $sector_size
  }
}

// next sector in chain:
// *  major version 3 must be 128 fields specified to fill a 512 byte sector
// *  major version 4 must be 1024 fields specified to fill a 4096 byte sector
// last fat sector can have entries that span past size of compound file. 
// Entries past the end of file, MUST be marked with FREESECT. file size is determined
// by the index of the last non-free FAT array entry.
// file size must be (N + 1) x (Sector Size) if FAT[N] != FREESECT

//////////////////////////////////////////////////////////////////////////////////////////////
// MINIFAT
// sector size of minifat is 64 bytes. 
// allocates space for the mini stream.
// if all streams in file are greater than the cutoff, mini fat and mini stream are not
// required and the start and root location starting sector can be set to ENDOFCHAIN
//////////////////////////////////////////////////////////////////////////////////////////////
macro_rules! mini_sector_number_to_byte_offset {
  ($sector_num:u8) => {
    $sector_num * 64
  }
}

// next sector in mini chain: see above.

//////////////////////////////////////////////////////////////////////////////////////////////
// DIFAT
// array of 32 bit sector numbers, stored in both the header and difat sectors
// header array consists of 109 entries.
// in DIFAT sector the array occupies entire sector minus 4 bytes for difat sector chaining
//////////////////////////////////////////////////////////////////////////////////////////////

// fat sector location
// *  for version 3, must have 127 fields specified to fill a 512 byte sector minus the "Next DIFAT Sector Location" field
// *  for version 4, must have 1023 fields specified to fill a 4096 byte sector minus the "Next DIFAT Sector Location" field

// Next DIFAT sector location = 4 bytes (last DIFACT sector must set end of chain)
