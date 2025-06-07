import { PAGE_SIZE, writeU32 } from "./memory";
const regs = {
    flags: {
        zero: 0,
        pos: 0, // positive
        neg: 0 // negative
    },
    common: new Uint32Array(64)
};
export function start(prog, jsFuns = []) {
    let sp = 2 * PAGE_SIZE - 1;
    let ip = 0;
    while (true) {
        const opcode = prog[ip++];
        if (opcode == 0 /* Op.PUSH_CONST */) {
            const val = prog[ip++];
            sp -= 4;
            writeU32(sp, val);
        }
        else if (opcode == 17 /* Op.CALL_JS */) {
            const index = prog[ip++];
            const func = jsFuns[index];
            func(sp);
        }
        else if (opcode == 1 /* Op.MOV_CONST */) {
            const reg = prog[ip++];
            const val = prog[ip++];
            regs.common[reg] = val;
        }
        else if (opcode == 21 /* Op.BRANCH */) {
            const index = prog[ip++];
            ip = index;
        }
        else if (opcode == 20 /* Op.BRANCH_COND */) {
            const type = prog[ip++];
            const index = prog[ip++];
            if (type == 0 /* BranchType.EQ */ && regs.flags.zero)
                ip = index;
        }
        else if (opcode == 18 /* Op.CMP_REG_CONST */) {
            const reg = prog[ip++];
            const val = prog[ip++];
            const sub = regs.common[reg] - val;
            if (sub < 0)
                regs.flags.neg = 1;
            else if (sub == 0)
                regs.flags.zero = 1;
            else
                regs.flags.pos = 1;
        }
        else if (opcode == 22 /* Op.CLEAR_FLAGS */) {
            regs.flags.neg = 0;
            regs.flags.pos = 0;
            regs.flags.zero = 0;
        }
        else if (opcode == 10 /* Op.ADD_CONST */) {
        }
        else if (opcode == 11 /* Op.ADD_REG */) {
        }
        else if (opcode == 12 /* Op.SUB_CONST */) {
        }
        else if (opcode == 13 /* Op.SUB_REG */) {
        }
        else if (opcode == 14 /* Op.MULT_CONST */) {
        }
        else if (opcode == 15 /* Op.MULT_REG */) {
        }
        else if (opcode == 8 /* Op.INC */) {
        }
        else if (opcode == 9 /* Op.DEC */) {
        }
        else if (opcode == 6 /* Op.LDR_CONST */) {
        }
        else if (opcode == 7 /* Op.LDR_REG */) {
        }
        else if (opcode == 5 /* Op.STR_REG_TO_REG */) {
        }
        else if (opcode == 4 /* Op.STR_REG_TO_CONST */) {
        }
        else if (opcode == 3 /* Op.STR_CONST_TO_REG */) {
        }
        else if (opcode == 2 /* Op.STR_CONST_TO_CONST */) {
        }
    }
}
