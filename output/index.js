import { memory, readCString, readU32 } from "./memory";
import { start } from "./vm";
memory.set(0, new Uint8Array(4096));
memory.set(1, new Uint8Array(4096));
const page0 = memory.get(0);
const str = "Hello world";
const strAddress = 0x300;
for (let i = strAddress; i < strAddress + str.length; i++)
    page0[i] = str.charCodeAt(i - strAddress) & 0xFF;
const prog = [
    1 /* Op.MOV_CONST */, 0, 3,
    18 /* Op.CMP_REG_CONST */, 0, 3,
    20 /* Op.BRANCH_COND */, 0 /* BranchType.EQ */, 15,
    0 /* Op.PUSH_CONST */, strAddress,
    17 /* Op.CALL_JS */, 0, // console.log
    17 /* Op.CALL_JS */, 1, // exit
    0 /* Op.PUSH_CONST */, strAddress + 2,
    21 /* Op.BRANCH */, 11,
];
const functions = [
    function consoleLog(addr) {
        console.log(readCString(readU32(addr)));
    },
    function exit() {
        process.exit();
    }
];
start(prog, functions);
