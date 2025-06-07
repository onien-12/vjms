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
    // mov r0, #0
    Op.MOV_CONST, 0, 0,
    
    Op.PUSH_CONST, strAddress, // push strAddress
    Op.MOV_REG, 1, 0, // mov r1, r0
    Op.ADD_CONST, 1, 48, // add r1, 48
    Op.STR_REG_TO_CONST, strAddress, 1, // str strAddress, r1
    Op.CALL_JS_CONST, 0, // console.log
    Op.ADD_CONST, REG_SP, 4,
    Op.INC, 0, // inc r0

    Op.CMP_REG_CONST, 0, 5, // cmp r0, #5
    Op.BRANCH_COND_CONST, BranchType.LT, 3, // blt #3

    Op.CALL_JS_CONST, 1, // exit
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