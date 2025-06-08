.main:
    mov r0, 0
    mov r1, 0
.loop_start:
    mov r1, r0
    push 0x300
    add r1, r1, 48
    str 0x300, r1
    calljs 0
    add sp, sp, 4
    inc r0

    cmp r0, 5
    blt .loop_start
.finish:
    calljs 1