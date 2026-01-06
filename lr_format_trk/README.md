# TRK Format

Created for [LRA](https://github.com/jealouscloud/linerider-advanced), with continued use in derivative builds ([LRA:CE](https://github.com/RatherBeLunar/LRA-Community-Edition), [LRTran](https://github.com/Tran-Foxxo/LRTran), [LRO](https://github.com/LunaKampling/LROverhaul)).

*This spec is a modified version of [Conqu3red's spec](https://github.com/Conqu3red/TRK-Docs/blob/master/The-TRK-Format.md)*

## Headers

- **0x00:** Magic number `0x54524BF2` spelling out `TRKò` (4 bytes)
- **0x04:** Version (should have value `0x01`), 8 bit unsigned integer (1 byte)
- **0x05:** Length (F) of the feature string, 16 bit signed integer (2 bytes)
- **0x07:** Feature string of length (F): ASCII encoded list of features seperated by `;` (additionally, final feature has a `;` at the end)

## Features

### LRA, LRA:CE, LRO
- `REDMULTIPLIER` - Red lines have multipliers
- `SCENERYWIDTH` - Width values for scenery lines
- `6.1` - Grid version 6.1 (default 6.2)
- `SONGINFO` - Track contains song metadata
- `IGNORABLE_TRIGGER` - Track contains lines with zoom triggers
- `ZEROSTART` - Set rider's initial velocity to (0, 0)

### LRA:CE and LRO exclusive
- `REMOUNT` - Use LRA:CE style remounting implementation (default: no remount)
- `FRICTIONLESS` - Disable friction

## Song Info
Only present if the `SONGINFO` feature was included.

- **0x00:** [C# Encoded String](https://docs.microsoft.com/en-us/openspecs/sharepoint_protocols/ms-spptc/89cf867b-260d-4fb2-ba04-46d8f5705555) which is a string prefixed by a 7BitEncodedInt length:
> The 7BitEncodedInt is written out 7 bits at a time starting with the least significant bits. If the value will not fit in 7 bits the high bit of the byte is set to indicate there is another byte of data to be written. The value is then shifted 7 bits to the right and the next byte is written. If the value will fit in the seven bits the high byte is not set and it signals the end of the structure.
- The string contains song name and song offset (as a string representation of a float) seperated by `\r\n`

## Line Data
* **0x00:** Rider start position x, double precision float (8 bytes)
* **0x08:** Rider start position y, double precision float (8 bytes)
* **0x016:** Number of lines (N), 32 bit unsigned integer (4 bytes)

For N times:
- **0x00:** Line Type + Flags
  - **Bit 8** Line inverted, boolean
  - **Bit 7 - 6** Line extension:
    - `0`: None
    - `1`: Left
    - `2`: Right
    - `3`: Both
  - **Bits 5 - 1** Line type:
    - `0`: Scenery/Green
    - `1`: Standard/Blue
    - `2`: Acceleration/Red
- If line type is `Acceleration` and feature `REDMULTIPLIER` is present:
  - **0x01:** Red line multiplier, 8 bit unsigned integer (1 byte)
  
- Denote current byte position as **R**
- If line type is `Acceleration` or `Standard` and feature `IGNORABLE_TRIGGER` is present:
  - **R** Zoom trigger present, boolean (1 byte)
  - If zoom trigger is present:
    - **R + 1:** Target, single precision float (4 bytes)
    - **R + 4:** Frames, 16 bit signed integer (2 bytes)
    - This trigger fires when the line it is attached to is touched, taking `Frames` to reach `Target` zoom level
    
- Denote current byte position as **S**
- **S:** Line ID, 32 bit signed integer (4 bytes)
- If extension is not `None`
  - **S + 4:** Previous line ID (ignored), 32 bit signed integer (4 bytes)
  - **S + 8:** Next line ID (ignored), 32 bit signed integer (4 bytes)
  
- Denote current byte position as **T**
- If line type is `Scenery` and feature `SCENERYWIDTH` is present:
  - **T:** Line width, 8 bit unsigned integer (1 byte)
  - Divide by 10.0 to get the actual width value
  
- Denote current byte position as **U**
- **U:** X position of the line's first endpoint, double precision float (8 bytes)
- **U + 8:** Y position of the line's first endpoint, double precision float (8 bytes)
- **U + 16:** X position of the line's second endpoint, double precision float (8 bytes)
- **U + 24:** Y position of the line's second endpoint, double precision float (8 bytes)

# Metadata

This section may not be present in older saves of original LRA. If you have reached the end of the file, then there is no metadata section.

## Headers
- **0x00:** Magic Number 0x4D455441 Spelling out META (4 bytes)
- **0x04:** Number of metadata entries (N), 16 bit signed integer (2 bytes)

For N times:
- **0x00:** Length of metadata string (L), 16 bit signed integer (2 bytes)
- **0x02:** Metadata string, ASCII encoded string of length L. Contains a key-value pair of the structure `KEY=VALUE`. All values listed (such as ints or floats) are stored as their string representation. Below are some possible keys and value types:

### LRA, LRA:CE, LRO
- `STARTZOOM` - Initial camera zoom, single precision float, default is 4.0

### LRA:CE and LRO exclusive
- `YGRAVITY` - Y gravity of rider, single precision float, default is 1
- `XGRAVITY` - X gravity of rider, single precision float, default is 0
- `GRAVITYWELLSIZE` - Size of gravity wells, double precision float, default is 10.0
- `BGCOLORR` - Red value of background color, 32 bit signed integer, default is 244
- `BGCOLORG` - Green value of background color, 32 bit signed integer, default is 245
- `BGCOLORB` - Blue value of background color, 32 bit signed integer, default is 249
- `LINECOLORR` - Red value of line color, 32 bit signed integer, default is 0
- `LINECOLORG` - Green value of line color, 32 bit signed integer, default is 0
- `LINECOLORB` - Blue value of line color, 32 bit signed integer, default is 0
    - `TRIGGERS` - The list of each trigger's data, seperated by an `&`. Note again that all values are string representations and not bit representations.
    - For each trigger:
      - Each trigger stores its values seperated by `:`. Value representations:
      - **values[0]** The first value is always the trigger type, 32 bit signed integer:
        - `0`: Zoom
          - **values[1]:** Target zoom, single precision float
          - **values[2]:** Start frame, 32 bit signed integer
          - **values[3]:** End frame, 32 bit signed integer
        - `1`: Background Color
          - **values[1]:** Red value of background color, 32 bit signed integer
          - **values[2]:** Green value of background color, 32 bit signed integer
          - **values[3]:** Blue value of background color, 32 bit signed integer
          - **values[4]:** Start frame, 32 bit signed integer
          - **values[5]:** End frame, 32 bit signed integer
        - `2`: Line Color
          - **values[1]:** Red value of line color, 32 bit signed integer
          - **values[2]:** Green value of line color, 32 bit signed integer
          - **values[3]:** Blue value of line color, 32 bit signed integer
          - **values[4]:** Start frame, 32 bit signed integer
          - **values[5]:** End frame, 32 bit signed integer
