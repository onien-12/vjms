pub enum IrInstruction {
    Declaration(String)
}

pub struct Function {
    pub name: String,
    pub instructions: Vec<IrInstruction>
}