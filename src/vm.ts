import { Op, BranchType } from "./instruction";
import { PAGE_SIZE, readU16, readU32, writeU32 } from './memory';

export const REG_FLAGS = 127;
export const REG_SP = 126;
export const REG_IP = 125;

const regs = new Uint32Array(128);

const advance = (prog: Array<number>) => prog[regs[REG_IP]++];
const clearFlags = () => regs[REG_FLAGS] &= 0xF8;
const setup = () => regs[REG_SP] = 2 * PAGE_SIZE - 1;

const pushConst = (val: number) => {
    regs[REG_SP] -= 4;
    writeU32(regs[REG_SP], val);
}

const branchCond = (type: BranchType, index: number) => {
    if (type == BranchType.EQ && regs[REG_FLAGS] & (1 << 0))
        regs[REG_IP] = index;
    if (type == BranchType.LT && regs[REG_FLAGS] & (1 << 1))
        regs[REG_IP] = index;
    if (type == BranchType.GT && regs[REG_FLAGS] & (1 << 2))
        regs[REG_IP] = index;
    clearFlags();
}

const cmp = (reg: number, val: number) => {
    const sub = regs[reg] - val;
    if (sub < 0) regs[REG_FLAGS] |= 1 << 1;
    else if (sub == 0) regs[REG_FLAGS] = 1 << 0;
    else regs[REG_FLAGS] = 1 << 2;
}

export function start(prog: Array<number>, jsFuns: Array<Function> = []) {
    setup();

    while (true) {
        const opcode = advance(prog); 

        switch (opcode) {
            // push <const>
            case Op.PUSH_CONST: pushConst(advance(prog)); break;

            // call <reg/const>
            case Op.CALL_JS_CONST: jsFuns[advance(prog)].call(null, regs[REG_SP]); break;
            case Op.CALL_JS_REG: jsFuns[regs[advance(prog)]].call(null, regs[REG_SP]); break;

            // mov reg, <reg/const>
            case Op.MOV_CONST: regs[advance(prog)] = advance(prog); break;
            case Op.MOV_REG: regs[advance(prog)] = regs[advance(prog)]; break;

            // b <reg/const>
            case Op.BRANCH_CONST: regs[REG_IP] = advance(prog); break;
            case Op.BRANCH_REG: regs[REG_IP] = regs[advance(prog)]; break;

            // bc <const>(type), <reg/const>
            case Op.BRANCH_COND_CONST: branchCond(advance(prog), advance(prog)); break;
            case Op.BRANCH_COND_REG: branchCond(advance(prog), regs[advance(prog)]); break;
            
            // cmp <reg>, <reg/const>
            case Op.CMP_REG_CONST: cmp(advance(prog), advance(prog)); break;
            case Op.CMP_REG_REG: cmp(advance(prog), regs[advance(prog)]); break;

            // cli
            case Op.CLEAR_FLAGS: clearFlags(); break;

            // inc/dec <reg>
            case Op.INC: regs[advance(prog)] += 1; break;
            case Op.DEC: regs[advance(prog)] -= 1; break;

            // add/sub/mult Rd, Rs, Rs
            // add/sub/mult Rd, Rs, imm
            case Op.ADD_CONST: regs[advance(prog)] = regs[advance(prog)] + advance(prog); break;
            case Op.ADD_REG: regs[advance(prog)] = regs[advance(prog)] + regs[advance(prog)]; break;
            case Op.SUB_CONST: regs[advance(prog)] = regs[advance(prog)] - advance(prog); break;
            case Op.SUB_REG: regs[advance(prog)] = regs[advance(prog)] - regs[advance(prog)]; break;
            case Op.MULT_CONST: regs[advance(prog)] = regs[advance(prog)] * advance(prog); break;
            case Op.MULT_REG: regs[advance(prog)] = regs[advance(prog)] * regs[advance(prog)]; break;

            // str <reg/const>, <reg/const>
            case Op.STR_CONST_TO_CONST: writeU32(advance(prog), advance(prog)); break;
            case Op.STR_CONST_TO_REG: writeU32(regs[advance(prog)], advance(prog)); break;
            case Op.STR_REG_TO_CONST: writeU32(advance(prog), regs[advance(prog)]); break;
            case Op.STR_REG_TO_REG: writeU32(regs[advance(prog)], regs[advance(prog)]); break;

            // ldr <reg>, <reg/const>
            case Op.LDR_CONST: regs[advance(prog)] = readU32(advance(prog)); break;
            case Op.LDR_REG: regs[advance(prog)] = readU32(regs[advance(prog)]); break;

            // xor/or/and Rd, Rs, Rs : ex. xor R0, r2, r3 => r0 = r2 ^ r3
            // xor/or/and Rd, Rs, imm
            case Op.XOR_REG: regs[advance(prog)] = regs[advance(prog)] ^ regs[advance(prog)]; break;
            case Op.XOR_CONST: regs[advance(prog)] = regs[advance(prog)] ^ advance(prog); break;
            case Op.OR_REG: regs[advance(prog)] = regs[advance(prog)] | regs[advance(prog)]; break;
            case Op.OR_CONST: regs[advance(prog)] = regs[advance(prog)] | advance(prog); break;
            case Op.AND_REG: regs[advance(prog)] = regs[advance(prog)] & regs[advance(prog)]; break;
            case Op.AND_CONST: regs[advance(prog)] = regs[advance(prog)] & advance(prog); break;

            // shr/shl Rd, Rs, Rs
            // shr/shl Rd, Rs, imm
            case Op.SHR_REG: regs[advance(prog)] = regs[advance(prog)] >> regs[advance(prog)]; break;
            case Op.SHR_CONST: regs[advance(prog)] = regs[advance(prog)] >> advance(prog); break;
            case Op.SHL_REG: regs[advance(prog)] = regs[advance(prog)] << regs[advance(prog)]; break;
            case Op.SHL_CONST: regs[advance(prog)] = regs[advance(prog)] << advance(prog); break;

            // not Rd, Rs
            // not Rd, imm
            case Op.NOT_REG: regs[advance(prog)] = ~regs[advance(prog)]; break;
            case Op.NOT_CONST: regs[advance(prog)] = ~advance(prog); break;
        }
    }
}