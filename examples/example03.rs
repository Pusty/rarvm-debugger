#include <constants.rh>

;This time the initial state of r0 is set to 0xC0FFEE
_start:
	cmp  r0, #0xC0FFEE
	jz   $secret
	mov  [#0x1000], #0x2e2e6f4e
    jmp  $done
secret:
	mov  [#0x1000], #0x21b39ab7
	xor  [#0x1000], r0

done:
    mov     [VMADDR_NEWBLOCKPOS],  #0x1000   ; Pointer  
    mov     [VMADDR_NEWBLOCKSIZE], #4       ; Size 
    jmp     #0x40000