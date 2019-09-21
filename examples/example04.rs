#include <constants.rh>

;This time the initial state of r0 = 0x0000006f, r1 = 0x6c000000, r2 = 0x006c0000, r3 = 0x00006500, r4 = 0x00000048
_start:
    add r1, r2
    add r1, r3
    add r1, r4
	mov  [#0x1000], r1
    mov  [#0x1004], r0
    mov     [VMADDR_NEWBLOCKPOS],  #0x1000   ; Pointer  
    mov     [VMADDR_NEWBLOCKSIZE], #5       ; Size 
    jmp     #0x40000