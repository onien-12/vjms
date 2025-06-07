export const PAGE_SIZE = 4096;
export const getPage = (addr) => Math.floor(addr / PAGE_SIZE);
export const getPageOffset = (addr) => addr % PAGE_SIZE;
export const memory = new Map();
export function readU8(addr) {
    const region = memory.get(getPage(addr));
    const offset = getPageOffset(addr);
    return region[offset];
}
export function readU16(addr) {
    return ((readU8(addr + 1) << 8) & 0xFF) |
        (readU8(addr) & 0xFF00);
}
export function readU32(addr) {
    return ((readU8(addr + 3) << 24) & 0xFF000000) |
        ((readU8(addr + 2) << 16) & 0xFF0000) |
        ((readU8(addr + 1) << 8) & 0xFF00) |
        (readU8(addr) & 0xFF);
}
export function writeU8(addr, value) {
    const region = memory.get(getPage(addr));
    const offset = getPageOffset(addr);
    region[offset] = value;
}
export function writeU16(addr, value) {
    writeU8(addr + 1, (value >> 8) & 0xFF);
    writeU8(addr, value & 0xFF);
}
export function writeU32(addr, value) {
    writeU8(addr + 3, (value >> 24) & 0xFF);
    writeU8(addr + 2, (value >> 16) & 0xFF);
    writeU8(addr + 1, (value >> 8) & 0xFF);
    writeU8(addr, value & 0xFF);
}
export function readCString(addr) {
    const strRegion = memory.get(getPage(addr));
    let str = "", char;
    while ((char = strRegion[getPageOffset(addr++)]) != 0)
        str += String.fromCharCode(char);
    return str;
}
