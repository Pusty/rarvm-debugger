#include <constants.rh>

;The output is defined as static included data and copied over to the output
_start:
    mov r0, #0x3C040
    mov r1, #0x1000
    
; Loop over static data until 0 byte is found
repeat:
    mov r2, [r0]
    and r2, #0xFF
    or  [r1], r2
    inc r0
    inc r1
    cmp r2, #0x00
    jnz $repeat
    
; Turn r1 into the length of the embedded string
    sub r1, #0x1001

; Setup output values to the position
    mov  [VMADDR_NEWBLOCKPOS],  #0x1000  ; Pointer  
    mov  [VMADDR_NEWBLOCKSIZE], r1       ; Size 
    jmp  #0x40000