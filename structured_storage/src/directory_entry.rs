// Compound File Directory Entry
// regular stream id = 0x00000000 - 0xfffffff9
let max_reg_stream_id: [u8; 4] = [0xff, 0xff, 0xff, 0xfa];
let no_stream: [u8; 4] = [0xff; 4];

// directory entry size is fixed a 128 bytes. Name limited to 32 UTF-16 code points including null terminator
// v3 groups 4 entries per sector
// v4 groups 32 entries per sector

enum ObjectType = {
  unknown_or_unallocated = 0x00,
  storage_object = 0x01,
  stream_object = 0x02,
  root_storage_object = 0x05
}

enum ColorFlag = {
  red = 0x00,
  black = 0x01
}

struct DirectoryEntry {
  entry_name: [u8, 64], // cannot contain /\:! 64 bit, utf-16 (32 characters in case I forget)
  entry_name_length: [u8; 2],
  object_type: ObjectType,
  color_flag: ColorFlag,
  left_sibling_id: [u8, 4],
  right_sibling_id: [u8, 4],
  child_id: [u8, 4],
  object_class_guid: [u8, 16], // CLSID guid
  state_bits: [u8, 4],
  creation_time: [u8, 8],
  modified_time: [u8, 8],
  starting_sector_location: [u8, 4],
  stream_size: [u8, 8]
}

// DirectoryEntry array, first entry is the root
// stream size:
// - for 512 sector: upper 32 bit field must b e set to zero when file is written. must accept uninitialized high DWORD to support older versions
// - for v4 4096 by sector: stream size is full 64 bit integer stream size
// unused directory entrys are marked object type 0x0, child right and left pointeres are initialized with NOSTREAM, everything else is 0

// red-black tree:
// 1. root is always black (color is actually irrelevant, this is arbitrary but goth)
// 2. no consecutive red nodes
// 3. left sibling must be less than right sibling
//    * node with a shorter name is less than one with alonger name (directory entry name length)
//    * if names are equal length, convert the names to uppercase using case conversion algorithm simple case variant/folding. compare each utf-16 code point
//    * unicode surrogate characters are never uppercased as they are represented by 2 utf-16 code points. Sorting relationship uppercases a single code point at a time
//    * lowercase characters defined by a newer, later version of the standard can be uppercased by that standard's case conversion implementation
