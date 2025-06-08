import { memory, readCString, readU32, writeU32 } from "./memory"
import { Op, BranchType } from "./instruction"
import { REG_SP, start } from "./vm"

memory.set(0, new Uint8Array(4096))
memory.set(1, new Uint8Array(4096))

const page0 = memory.get(0)
const str = "Hello world" 
const strAddress = 0x300;

for (let i = strAddress; i < strAddress + str.length; i++) 
    page0[i] = str.charCodeAt(i - strAddress) & 0xFF;

const prog = [
 Op.MOV_CONST, 0, 0,
Op.MOV_CONST, 1, 0,
Op.MOV_REG, 1, 0,
Op.PUSH_CONST, 768,
Op.ADD_CONST, 1, 1, 48,
Op.STR_REG_TO_CONST, 768, 1,
Op.CALL_JS_CONST, 0,
Op.ADD_CONST, 126, 126, 4,
Op.INC, 0,
Op.CMP_REG_CONST, 0, 5,
Op.BRANCH_COND_CONST, BranchType.LT, 6,
Op.CALL_JS_CONST, 1,
]

const functions = [
    function consoleLog(esp: number) {
        console.log(readCString(readU32(esp)))
    },
    function exit() {
        process.exit();
    }
]


start(prog, functions)