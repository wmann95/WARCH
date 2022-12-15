# WARCH

## Description
A rust implementation of an emulated computer. It currently has 14 instruction op-codes that can be used. Currently implemented as a hardcoded 32-bit system. This will be changed in the future in order to make system configuration possible.

## Usage
In order to use this, ensure your Rust is up-to-date and Cargo is functional. Clone the repository to a convenient location and run "cargo build -r" from the project folder.

A file of binary instructions is required to run WARCH. this is done by using "./target/release/WARCH -i [FILENAME]".

WARCH also comes built with a disassembler. Use "./target/release/WARCH -d [FILENAME]". This will print to stdout the mnemonic used for the opcode along with ra, rb, and rc when appropriate, and the load register and load value for movi.

## Instructions

### Instruction Set
|   Opcode   | mnemonic | Name | Description |
|   ------   | -------- | ---- | ----------- |
| 0 | cmov | Conditional Move | if $r[C] != 0; $r[A] = $r[B] |
| 1 | load | segmented Load | $r[A] = $m[$r[B]][$r[C]] |
| 2 | store |Segmented Store | $m[$r[A]][$r[B]] = $r[C] |
| 3 | add | Add | $r[A] = ($r[B] + $r[C]) mod 2^32 |
| 4 | mul | Multiply | $r[A] = ($r[B] * $r[C]) mod 2^32 |
| 5 | div | Divide | $r[A] = ($r[B] / $r[C]) mod 2^32 |
| 6 | nand | Bitwise NAND | $r[A] = ~($r[B] & $r[C]) |
| 7 | halt | Halt | Computation Stops |
| 8 | map | Map Segment | Creates a new memory segment with word length equal to the value in $r[c]. The identifier is stored in $r[b]. The segment is mapped as $m[$r[b]] |
| 9 | umap | Unmap Segment | Frees the memory segment identified by the value in $r[c]. |
| 10 | output | Output | The value in $r[c] is displayed to the I/O device immediately. Values must be in range of 0 to 255. |
| 11 | input | Input | Await input from I/O device. When it arrives, $r[c] is loaded with the value. If EOF is found, $r[c] is loaded with u32::MAX. |
| 12 | run | Load Program | Segment $m[$r[b]] is duplicated, which then replaces the current segment $m[0]. the program counter is then set to the value in $r[c]. If $r[b] is 0, then this is a jump in the current program. |
| 13 | movi | Load Value | Immediate Load. See semantics below. |

#### Load Value
The first four bits of the word denote the opcode. The next 3 bits denote which register to load the value into. The final 25 bits denote the value to be stored.
