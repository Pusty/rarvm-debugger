# RARVM-Debugger

The RARVM-Debugger is a dynamic debugging and diagnostic tool for the RARVM made out of the modified source code of unrar version 4.2.1.

## Usage

### Execute an Archive

`./rarvm-debugger p -inul <archive.rar>`

Output the content of the archive after executing it through the RarVM to stdout.

```
./rarvm-debugger p -inul example03.rar 
Yes!
```

### Debug an Archive

`./rarvm-debugger d <archive.rar>`

```
(dbg) help
                          HELP DIALOG                          
===============================================================
 help                            - help dialog
 info registers                  - information about registers
 break  [<addr>]                 - set a breakpoint
 delete [<addr>]                 - remove breakpoints
 disassemble [<amount>] [<from>] - disassemble instructions
 get flags                       - read the flags
 get R<register>                 - read a register
 get <addr>                      - read byte from memory
 set flags <value>               - set the flags
 set R<register>  <value>        - set a register to a value
 set <addr> <value>              - set a byte in memory
 stepi <amount>                  - step over instructions
 continue                        - continue until next break
 quit                            - exit the debugger
```

### Trace the execution of an Archive

`./rarvm-debugger d -trace <archive.rar>`

Output an instruction trace of the execution of a RARVM program.

```
  ./rarvm-debugger d -trace example01.rar 
  [0000] JMP      #0x00000001
  [0001] MOVD     R0, #0x0000BEEF
  [0002] CMPD     R0, #0x00C0FFEE
  [0003] JZ       #0x00000006
  [0004] MOVD     [#0x00001000], #0x2E2E6F4E
  [0005] JMP      #0x00000008
  [0008] MOVD     [#0x0003C020], #0x00001000
  [0009] MOVD     [#0x0003C01C], #0x00000004
  [000A] JMP      #0x0003FF00
```

### Dump the code of an Archive

`./rarvm-debugger d -dump <archive.rar>`

Dump all code loaded in the RARVM without executing it.

```
  ./rarvm-debugger d -dump example01.rar 
  JMP      #0x00000001
  MOVD     R0, #0x0000BEEF
  CMPD     R0, #0x00C0FFEE
  JZ       #0x00000006
  MOVD     [#0x00001000], #0x2E2E6F4E
  JMP      #0x00000008
  MOVD     [#0x00001000], #0x21B39AB7
  XOR      [#0x00001000], R0
  MOVD     [#0x0003C020], #0x00001000
  MOVD     [#0x0003C01C], #0x00000004
  JMP      #0x0003FF00
  MOVD     #0x00000000, #0x00000000
  RETb 
```

### Print out Diagnostic Information

`./rarvm-debugger d -diagnostic <archive.rar>`

Output Debug information regarding the parsing of the RarVM programs parameters and execute the program.
Outputs nothing if the archive does not contain RarVM code.

```
  ./rarvm-debugger d -diagnostic example04.rar 
  FirstByte = 17
  Length = 4E
    Initial Block Start: 00000000
    Default Block Length: 00000000
    Modified Initial Register Flag: 1F
      Changed Initial Register 0 to 0000006F
      Changed Initial Register 1 to 6C000000
      Changed Initial Register 2 to 006C0000
      Changed Initial Register 3 to 00006500
      Changed Initial Register 4 to 00000048
    Code Size: 48
  The checksum (B4) matches the expected value (B4).
  Initial R6 Value = 0
  Initial GlobalData[0x24] = 0
  Initial GlobalData[0x28] = 0
  Initial R7 Value = 262144
```

## Licence

UnRAR licence is included in the source folder.


# RARVM

*Note that the RARVM was removed from unrar and winrar in the release of version 5.0 so the information displayed here has very few practical application*

## Registers

The Architecture provides 8 General Purpose Registers R0..R7, a program counter and a flag register containing a carry, zero and signed flag.
R7 is used as a stack pointer by the instructions using the stack.

## Instruction Set

| Opcode | Mnemonic | Parameters | Encoding              |  Description    | 
|--------|----------|------------|-----------------------|-----------------|
| 00     | MOV[b]   | Rd, Rs     | [0][000][b][Rd][Rs]   | Move Rs into Rd  |
| 01     | CMP[b]   | Ra, Rb     | [0][001][b][Ra][Rb]   | Compare Ra to Rb |
| 02     | ADD[b]   | Rd, Rs     | [0][010][b][Rd][Rs]   | Add Rs to Rd  |
| 03     | SUB[b]   | Rd, Rs     | [0][011][b][Rd][Rs]   | Subtract Rs from Rd  |
| 04     | JZ       | dest       | [0][100][dest]        | Jump if the zero flag is set  |
| 05     | JNZ      | dest       | [0][101][dest]        | Jump if the zero flag is not set  |
| 06     | INC[b]   | Rd         | [0][110][b][Rd]       | Increase Rd by one  |
| 07     | DEC[b]   | Rd         | [0][111][b][Rd]       | Decrease Rd by one  |
| 08     | JMP      | dest       | [1][00000][dest]      | Jump   |
| 09     | XOR[b]   | Rd, Rs     | [1][00001][b][Rd][Rs] | Rd = Rd ^ Rs  |
| 0A     | AND[b]   | Rd, Rs     | [1][00010][b][Rd][Rs] | Rd = Rd & Rs  |
| 0B     | OR[b]    | Rd, Rs     | [1][00011][b][Rd][Rs] | Rd = Rd \| Rs  |
| 0C     | TEST[b]  | Ra, Rb     | [1][00100][b][Ra][Rb] | Bitwise compare Ra to Rb  |
| 0D     | JS       | dest       | [1][00101][dest]      | Jump if the signed flag is set  |
| 0E     | JNS      | dest       | [1][00110][dest]      | Jump if the signed flag is not set  |
| 0F     | JB       | dest       | [1][00111][dest]      | Jump if the carry flag is set  |
| 10     | JBE      | dest       | [1][01000][dest]      | Jump if the carry flag or the zero flag is set  |
| 11     | JA       | dest       | [1][01001][dest]      | Jump if the carry flag and the zero flag are not set  |
| 12     | JAE      | dest       | [1][01010][dest]      | Jump if the carry flag is not set  |
| 13     | PUSH     | Rs         | [1][01011]            | Subtract 4 from R7, Put Rs on the stack pointed to by R7   |
| 14     | POP      | Rd         | [1][01100]            | Put the value from the stack pointed to by R7 to Rd, Increase R7 by 4  |
| 15     | CALL     | dest       | [1][01101][dest]      | Subtract 4 from R7, Put the return address to the stack, Jump  |
| 16     | RET      |            | [1][01110]            | Set the instruction pointer to the value pointed to on stack, Increase R7 by 4  |
| 17     | NOT[b]   | Rd         | [1][01111][b][Rd]     | Rd = ~Rd  |
| 18     | SHL[b]   | Rd, Rs     | [1][10000][b][Rd][Rs] | Shift Rd left by Rs  |
| 19     | SHR[b]   | Rd, Rs     | [1][10001][b][Rd][Rs] | Logical shift Rd right by Rs  |
| 1A     | SAR[b]   | Rd, Rs     | [1][10010][b][Rd][Rs] | Arithmetic shift Rd right by Rs |
| 1B     | NEG[b]   | Rd         | [1][10011][b][Rd]     | Rd = -Rd  |
| 1C     | PUSHA    |            | [1][10100]            | Push all registers on the stack  |
| 1D     | POPA     |            | [1][10101]            | Pop all registers from the stack  |
| 1E     | PUSHF    |            | [1][10110]            | Push the flags on the stack  |
| 1F     | POPF     |            | [1][10111]            | Pop the flags from the stack  |
| 20     | MOVZX    | Rd, Rs     | [1][11000]            | Move a byte from Rs to Rd and zero fill the rest  |
| 21     | MOVSX    | Rd, Rs     | [1][11001]            | Move a byte from Rs to Rd and fill the rest depending on the sign  |
| 22     | XCHG[b]  | Rd, Rs     | [1][11010][b][Rd][Rs] | TMP = Rd; Rd = Rs; Rs = TMP  |
| 23     | MUL[b]   | Rd, Rs     | [1][11011][b][Rd][Rs] | Rd = Rd * Rs  |
| 24     | DIV[b]   | Rd, Rs     | [1][11100][b][Rd][Rs] | Rd = Rd / Rs  |
| 25     | ADC[b]   | Rd, Rs     | [1][11101][b][Rd][Rs] | Rd = Rd + Rs + Carry Bit  |
| 26     | SBB[b]   | Rd, Rs     | [1][11110][b][Rd][Rs] | Rd = Rd - Rs - Carry Bit  |
| 27     | PRINT    |            | [1][11111]            | Do nothing (see notes)  |

[b] is the ByteMode bit which decides whether an Instruction operates on a 4 bytes or if the bit is set only byte at a time.

For the other parameters the decoding function can be read here:
https://github.com/pmachapman/unrar/blob/bca1c247dd58da11e500013130a22ca64e830a55/rarvm.cpp#L674
It's notable that all operants can be registers, immediate values or memory references from a register, an offset or both.

## Environment

I've done most of my testing with unrar 4.2.1 and WinRar 4.20

## Notes

Since Version 3.3.0 instruction parameters data pointers point to the content of them by default, allow some cases forms of self modifying code
```
./rarvm-debugger d -trace example02.rar 
[0000] SUB      #0x00000002, #0x00000001
[0001] JNZ      #0x00000001
[0000] SUB      #0x00000001, #0x00000001
[0001] JNZ      #0x00000001
```
https://github.com/pmachapman/unrar/commit/614508aaf441d08de64d68bf3464ecfbd9b603c9#diff-9fbcab26b4523426ccb1520539e7408b

In Version 3.6.1 the "PRINT" debug instruction was removed
https://github.com/pmachapman/unrar/commit/30566e3abf4c9216858bae3ea6b44f048df8c4a5#diff-9fbcab26b4523426ccb1520539e7408b


# Examples

## Example 01

A binary which output depends on the initial register state, without tools or modification will always output "No.."

## Example 02

An example of self modifying instructions (See notes)

## Example 03

Example01 but with changed initial register state so it will output "Yes!" by default

## Example 04

Multiple changed initial register states that contain the output at startup

## Example 05

An example of included static binary data. It copies over the static data to output it


## References

- https://github.com/taviso/rarvmtools
- http://blog.cmpxchg8b.com/2012/09/fun-with-constrained-programming.html
- https://github.com/pmachapman/unrar
- https://github.com/pmachapman/unrar/tree/bca1c247dd58da11e500013130a22ca64e830a55
- https://github.com/pmachapman/unrar/blob/bca1c247dd58da11e500013130a22ca64e830a55/rarvm.cpp
- https://github.com/pmachapman/unrar/blob/bca1c247dd58da11e500013130a22ca64e830a55/rarvm.hpp
- https://github.com/pmachapman/unrar/blob/bca1c247dd58da11e500013130a22ca64e830a55/rarvmtbl.cpp