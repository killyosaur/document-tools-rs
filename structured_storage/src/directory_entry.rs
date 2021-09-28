// Compound File Directory Entry
// regular stream id = 0x00000000 - 0xfffffff9
let max_reg_stream_id: [u8; 4] = [0xff; 0xff; 0xff; 0xfa];
let no_stream: [u8; 4] = [0xff; 0xff; 0xff; 0xff];

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
  entry_name: String, // cannot contain /\:!
  entry_name_length: [u8; 2],
  object_type: ObjectType,
  color_flag: ColorFlag,
  left_sibling_id: [u8, 4],
  right_sibling_id: [u8, 4],
  child_id: [u8, 4],
  object_class_guid: 
}
