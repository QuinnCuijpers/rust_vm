.fib 
    LDI r1 8 
    LDI r2 0
    LDI r3 1
    LDI r4 0
.loop 
    DEC r1
    BRH nc .done
    ADD r3 r0 r2
    ADD r4 r0 r3
    ADD r2 r3 r4
    JMP .loop
.done 
    HLT