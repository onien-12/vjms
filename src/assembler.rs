use std::fmt;

const REG_IP: u16 = 125;
const REG_SP: u16 = 126;
const REG_FLAGS: u16 = 127;

pub enum Cond {
    LT, GT,
    EQ, NEQ
}

pub enum Arg {
    Reg(u16),
    Imm(u32),
    Label(String)
}

pub enum Instruction {
    Mov(Arg, Arg),
    Cmp(Arg, Arg),
    Branch(Arg),
    BranchCond(Cond, Arg),
    Call(Arg),
    Calljs(Arg),
    Push(Arg),
    Str(Arg, Arg),

    Add(Arg, Arg, Arg),
    Inc(Arg),
    Dec(Arg),
    Cli()
}

pub struct Label {
    pub name: String,
    pub instructions: Vec<Instruction>
}

pub struct Program {
    pub labels: Vec<Label>,
    pub tok: usize
}

pub trait Parser {
    fn skip_until(&mut self, program: &String, c: char) -> String;
    fn skip_until_whitespace(&mut self, program: &String) -> String;
    fn skip_whitespace(&mut self, program: &String);
    fn next_arg(&mut self, program: &String, is_last: bool) -> Arg;
    fn parse_register(&mut self, reg: String) -> u16;
    fn parse_arg(&mut self, arg: String) -> Arg;
    fn parse(&mut self, program: String);
}

fn arg_count(insn: &Instruction) -> usize {
    match insn {
        Instruction::Add(_, _, _) => 3,

        Instruction::Mov(_, _) | Instruction::Cmp(_, _) | Instruction::BranchCond(_, _) |
        Instruction::Str(_, _) => 2,

        Instruction::Inc(_) | Instruction::Dec(_) |  Instruction::Branch(_) |
        Instruction::Push(_) | Instruction::Call(_) | Instruction::Calljs(_) => 1,

        Instruction::Cli() => 0
    }
}

impl Program {
    pub fn new() -> Self {
        Self { labels: vec![], tok: 0 }
    }

    pub fn find_label_start_index(&self, needle: &String) -> usize {
        let mut result = 0usize;
        for label in &self.labels {
            if label.name == *needle {
                return result;
            }
            for insn in &label.instructions {
                result += arg_count(insn) + 1;
            }
        }

        0
    }

    pub fn assemble(&self) -> String {
        let mut result = String::new();

        for label in &self.labels {
            for insn in &label.instructions {
                match insn {
                    Instruction::Mov(arg1, arg2) => {
                        if let Arg::Reg(dest) = arg1 {
                            match arg2 {
                                Arg::Imm(value) => result += format!("Op.MOV_CONST, {}, {},\n", dest, value).as_str(),
                                Arg::Reg(reg) => result += format!("Op.MOV_REG, {}, {},\n", dest, reg).as_str(),
                                Arg::Label(_) => panic!("ERROR: Wrong source type for mov")
                            }
                        } else {
                            panic!("ERROR: Wrong destination type for mov")
                        }
                    },
                    Instruction::Branch(arg) => {
                        if let Arg::Label(target) = arg {
                            result += format!("Op.BRANCH_CONST, {},\n", self.find_label_start_index(target)).as_str();
                        } else if let Arg::Reg(reg) = arg {
                            result += format!("Op.BRANCH_REG, {},\n", reg).as_str();
                        } else {
                            panic!("ERROR: Wrong argument type for branch")
                        }
                    },
                    Instruction::BranchCond(cond, arg) => {
                        let cond_str = match cond {
                            Cond::EQ => "BranchType.EQ",
                            Cond::GT => "BranchType.GT",
                            Cond::LT => "BranchType.LT",
                            Cond::NEQ => "BranchType.NEQ"
                        };

                        if let Arg::Label(target) = arg {
                            result += format!("Op.BRANCH_COND_CONST, {}, {},\n", cond_str, self.find_label_start_index(target)).as_str();
                        } else if let Arg::Reg(reg) = arg {
                            result += format!("Op.BRANCH_COND_REG, {}, {},\n", cond_str, reg).as_str();
                        } else {
                            panic!("ERROR: Wrong argument type for branch with condition")
                        }
                    },
                    Instruction::Cmp(arg1, arg2) => {
                        if let Arg::Reg(reg1) = arg1 {
                            if let Arg::Imm(value) = arg2 {
                                result += format!("Op.CMP_REG_CONST, {}, {},\n", reg1, value).as_str();
                            } else if let Arg::Reg(reg2) = arg2 {
                                result += format!("Op.CMP_REG_REG, {}, {},\n", reg1, reg2).as_str();
                            } else {
                                panic!("ERROR: Wrong argument type for cmp (argument 1 must be a reg or imm)")
                            }
                        } else {
                            panic!("ERROR: Wrong argument type for cmp (argument 0 must be a reg)")
                        }
                    },
                    Instruction::Add(arg1, arg2, arg3) => {
                        if let Arg::Reg(dst) = arg1 {
                            if let Arg::Reg(reg1) = arg2 {
                                if let Arg::Reg(reg2) = arg3 {
                                    result += format!("Op.ADD_REG, {}, {}, {},\n", dst, reg1, reg2).as_str();
                                } else if let Arg::Imm(value) = arg3 {
                                    result += format!("Op.ADD_CONST, {}, {}, {},\n", dst, reg1, value).as_str();
                                } else {
                                    panic!("ERROR: Wrong argument type for add (argument 2 must be a reg or imm)")
                                }
                            } else {
                                panic!("ERROR: Wrong argument type for add (argument 1 must be a reg)")
                            }
                        } else {
                            panic!("ERROR: Wrong argument type for add (argument 0 must be a reg)")
                        }
                    },
                    Instruction::Push(arg1) => {
                        if let Arg::Reg(reg) = arg1 {
                            result += format!("Op.PUSH_REG, {},\n", reg).as_str();
                        } else if let Arg::Imm(value) = arg1 {
                            result += format!("Op.PUSH_CONST, {},\n", value).as_str();
                        } else {
                            panic!("ERROR: Wrong argument type for push (argument 0 must be a reg or imm)")
                        }
                    },
                    Instruction::Str(arg1, arg2) => {
                        if let Arg::Reg(dst_reg) = arg1 {
                            if let Arg::Reg(src_reg) = arg2 {
                                result += format!("Op.STR_REG_TO_REG, {}, {},\n", dst_reg, src_reg).as_str();
                            } else if let Arg::Imm(src_value) = arg2 {
                                result += format!("Op.STR_CONST_TO_REG, {}, {},\n", dst_reg, src_value).as_str();
                            } else {
                                panic!("ERROR: Wrong argument type for str (argument 1 must be a reg or imm)")
                            }
                        } else if let Arg::Imm(dst_value) = arg1 {
                            if let Arg::Reg(src_reg) = arg2 {
                                result += format!("Op.STR_REG_TO_CONST, {}, {},\n", dst_value, src_reg).as_str();
                            } else if let Arg::Imm(src_value) = arg2 {
                                result += format!("Op.STR_CONST_TO_CONST, {}, {},\n", dst_value, src_value).as_str();
                            } else {
                                panic!("ERROR: Wrong argument type for str (argument 1 must be a reg or imm)")
                            }
                        } else {
                            panic!("ERROR: Wrong argument type for str (argument 0 must be a reg or imm)")
                        }
                    },
                    Instruction::Call(arg1) => {
                        if let Arg::Reg(reg) = arg1 {
                            result += format!("Op.CALL_REG, {},\n", reg).as_str();
                        } else if let Arg::Imm(value) = arg1 {
                            result += format!("Op.CALL_CONST, {},\n", value).as_str();
                        } else if let Arg::Label(label) = arg1 {
                            result += format!("Op.CALL_CONST, {},\n", self.find_label_start_index(label)).as_str();
                        }
                    },
                    Instruction::Calljs(arg1) => {
                        if let Arg::Reg(reg) = arg1 {
                            result += format!("Op.CALL_JS_REG, {},\n", reg).as_str();
                        } else if let Arg::Imm(value) = arg1 {
                            result += format!("Op.CALL_JS_CONST, {},\n", value).as_str();
                        } else {
                            panic!("ERROR: Wrong argument type for calljs (argument 0 must be a reg or imm)")
                        }
                    },
                    Instruction::Inc(arg) => {
                        if let Arg::Reg(reg) = arg {
                            result += format!("Op.INC, {},\n", reg).as_str();
                        } else {
                            panic!("ERROR: Wrong argument type for inc")
                        }
                    },
                    Instruction::Dec(arg) => {
                        if let Arg::Reg(reg) = arg {
                            result += format!("Op.DEC, {},\n", reg).as_str();
                        } else {
                            panic!("ERROR: Wrong argument type for dec")
                        }
                    },
                    Instruction::Cli() => {
                        result += format!("Op.CLEAR_FLAGS,\n").as_str();
                    }
                }
            }
        }

        result
    }   
}

impl Parser for Program {
    fn skip_until(&mut self, program: &String, until: char) -> String {
        let mut c = program.chars().nth(self.tok).unwrap();
        let mut result = String::new();
        while c != until {
            result.push(c);
            self.tok += 1;

            if self.tok < program.len() {
                c = program.chars().nth(self.tok).unwrap();
            } else {
                break;
            }
        }

        result
    }

    fn skip_until_whitespace(&mut self, program: &String) -> String {
        let mut c = program.chars().nth(self.tok).unwrap();
        let mut result = String::new();
        while c != ' ' && c != '\t' && c != '\n' && self.tok < program.len()  {
            result.push(c);
            self.tok += 1;
            if self.tok < program.len() {
                c = program.chars().nth(self.tok).unwrap();
            } else {
                break;
            }
        }

        result
    }

    fn skip_whitespace(&mut self, program: &String) {
        if self.tok >= program.len() - 1 { return }
        let mut c = program.chars().nth(self.tok).unwrap();
        while c == '\n' || c == '\t' || c == ' ' {
            self.tok += 1;
            c = program.chars().nth(self.tok).unwrap();
        }
    }

    fn parse_register(&mut self, reg: String) -> u16 {
        reg[1..].parse::<u16>().unwrap()
    }

    fn parse_arg(&mut self, arg: String) -> Arg {
        match arg.as_str() {
            "ip" => Arg::Reg(REG_IP),
            "flgs" => Arg::Reg(REG_FLAGS),
            "sp" => Arg::Reg(REG_SP),
            _ => match arg.chars().nth(0).unwrap() {
                '.' => Arg::Label(arg[1..].to_string()),
                'r' => Arg::Reg(self.parse_register(arg)),
                '0' => {
                    if arg.len() > 1 {
                        match arg.chars().nth(1).unwrap() {
                            'x' => Arg::Imm(u32::from_str_radix(&arg[2..], 16).unwrap()),
                            'o' => Arg::Imm(u32::from_str_radix(&arg[2..], 8).unwrap()),
                            'b' => Arg::Imm(u32::from_str_radix(&arg[2..], 2).unwrap()),
                            _ => panic!("ERROR at parsing: wrong radix number: {}", arg),
                        }
                    } else {
                        Arg::Imm(0)
                    } 
                },
                '1' | '2' | '3' | '4' |
                '5' | '6' | '7' | '8' | '9' => 
                    Arg::Imm(arg.parse::<u32>().unwrap()),
                _ => panic!("ERROR at parsing: Wrong arg: {}", arg)
            }
        }
    }

    fn next_arg(&mut self, program: &String, is_last: bool) -> Arg {
        let c: char = if is_last { '\n' } else { ',' };
        let arg = self.skip_until(&program, c);
        if self.tok < program.len() - 1 {
            self.tok += 1; self.skip_whitespace(&program);
        }

        self.parse_arg(arg)
    }

    fn parse(&mut self, program: String) {
        while self.tok < program.len() {
            let c = program.chars().nth(self.tok).unwrap();
            if c == '.' {
                self.tok += 1;
                let label_name = self.skip_until(&program, ':');
                self.tok += 1; 

                let mut instructions: Vec<Instruction> = Vec::new();

                loop {
                    self.skip_whitespace(&program);

                    let instruction_name = self.skip_until_whitespace(&program);
                    self.skip_whitespace(&program);

                    match instruction_name.as_str() {
                        "add" => instructions.push(Instruction::Add(self.next_arg(&program, false), self.next_arg(&program, false), self.next_arg(&program, true))),
                        "mov" => instructions.push(Instruction::Mov(self.next_arg(&program, false), self.next_arg(&program, true))),
                        "str" => instructions.push(Instruction::Str(self.next_arg(&program, false), self.next_arg(&program, true))),
                        "cmp" => instructions.push(Instruction::Cmp(self.next_arg(&program, false), self.next_arg(&program, true))),
                        "b" =>  instructions.push(Instruction::Branch(self.next_arg(&program, true))),
                        "blt" => instructions.push(Instruction::BranchCond(Cond::LT, self.next_arg(&program, true))),
                        "bgt" => instructions.push(Instruction::BranchCond(Cond::GT, self.next_arg(&program, true))),
                        "beq" => instructions.push(Instruction::BranchCond(Cond::EQ, self.next_arg(&program, true))),
                        "bneq" => instructions.push(Instruction::BranchCond(Cond::NEQ, self.next_arg(&program, true))),
                        "dec" => instructions.push(Instruction::Dec(self.next_arg(&program, true))),
                        "inc" => instructions.push(Instruction::Inc(self.next_arg(&program, true))),
                        "push" => instructions.push(Instruction::Push(self.next_arg(&program, true))),
                        "call" => instructions.push(Instruction::Call(self.next_arg(&program, true))),
                        "calljs" => instructions.push(Instruction::Calljs(self.next_arg(&program, true))),
                        "cli" => instructions.push(Instruction::Cli()),
                        _ => panic!("No such instruction: {}", instruction_name)
                    }

                    if self.tok >= program.len() - 1 { break }
                    if let Some(c) = program.chars().nth(self.tok) { 
                        if c == '.' { break }
                    }
                }

                self.labels.push(Label {
                    instructions,
                    name: label_name.clone()
                });
            }
        }
    }
}

impl fmt::Display for Program {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for label in &self.labels {
            writeln!(f, "{}:", label.name)?;
            for instruction in &label.instructions {
                writeln!(f, "\t{}", instruction)?;
            }
        }

        Ok(())
    }
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::Add(dest, arg1, arg2) => write!(f, "Add({}, {}, {})", dest, arg1, arg2),
            Instruction::Mov(dest, src) => write!(f, "Mov({}, {})", dest, src),
            Instruction::Str(dest, src) => write!(f, "Str({}, {})", dest, src),
            Instruction::BranchCond(cond, arg) => write!(f, "BranchCond({}, {})", cond, arg),
            Instruction::Branch(target) => write!(f, "B({})", target),
            Instruction::Cmp(arg1, arg2) => write!(f, "Cmp({}, {})", arg1, arg2),
            Instruction::Inc(arg) => write!(f, "Inc({})", arg),
            Instruction::Dec(arg) => write!(f, "Dec({})", arg),
            Instruction::Push(arg) => write!(f, "Push({})", arg),
            Instruction::Calljs(arg) => write!(f, "Calljs({})", arg),
            Instruction::Call(arg) => write!(f, "Call({})", arg),
            Instruction::Cli() => write!(f, "Cli()"),
        }
    }
}

impl fmt::Display for Arg {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Arg::Imm(value) => write!(f, "#{}", value),
            Arg::Reg(value) => write!(f, "r{}", value),
            Arg::Label(label) => write!(f, ".{}", label)
        }   
    }
}

impl fmt::Display for Cond {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cond::EQ => write!(f, "Cond.EQ"),
            Cond::NEQ => write!(f, "Cond.NEQ"),
            Cond::LT => write!(f, "Cond.LT"),
            Cond::GT => write!(f, "Cond.GT"),
        }   
    }
}
