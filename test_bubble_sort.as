    LDI r15 9
.outer LDI r14 8
.inner LOD r14 r1 0
    LOD r14 r2 1
    CMP r1 r2 
    BRH nc .skip
    ADD r1 r0 r3
    ADD r2 r0 r1 
    ADD r3 r0 r2
.skip STR r14 r1 0
    STR r14 r2 1
    DEC r14
    BRH carry .inner
    DEC r15
    BRH carry .outer
    HLT

