bits 16

start:
    MOV AX, 5         ; AX = 5
    MOV BX, 3         ; BX = 3
loop:
    SUB AX, BX        ; AX -= BX
    JZ end            ; pokud AX == 0 -> end
    INC AX            ; AX++
    JMP loop          ; opakuj
end:
    MOV [0x1000], AX  ; ulož výsledek do paměti
    NOP
