#bank code

    MOVH r1, 0x01
    MOVH r2, 0x02
    MOVL r3, 0x10

loop:
    LDL r4, [r1]
    STL r4, [r2]
    INC r1
    INC r2
    DEC r3
    BNZ loop

    HLT

#bank data

#d8 2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53