; This program calculates the Fibonacci sequence using a loop.
; It stores each value in memory at an increasing address.
;
; Registers usage:
; - r1: last number
; - r2: current number
; - r3: memory pointer
; - r4: maximum value
; - r5: next number

#bank code

    MOVL r2, 1      ; r2 = F(1)
    MOVH r3, 0x01   ; r3 = 0x0100 (initial memory address)
    MOVL r4, 0xff   ; r4 = 0x00ff (maximum value)

    INC r3          ; increment memory pointer
    STL r2, [r3]    ; store F(1) in memory
    
loop:
    ADD r5, r1, r2  ; r5 = r1 + r2
    CMP r4, r5      ; check if next number is within range
    BS end          ; if not, jump to the end of the program
    MOV r1, r2      ; r1 = r2
    MOV r2, r5      ; r2 = r5
    INC r3          ; increment memory pointer
    STL r2, [r3]    ; store next number in memory
    JMP loop        ; loop again

end:
    HLT             ; halt the program