_start:
repeat:
    sub  #0x000003, #1
    jnz  $repeat
    jmp  #0x0003FF00
