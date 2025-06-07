use std::fmt;

pub enum Arg {
    Reg(u16),
    Imm(u32),
    Label(String)
}

pub enum Instruction {
    Mov(Arg, Arg),
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
    fn parse_register(&mut self, reg: String) -> u16;
    fn parse_arg(&mut self, arg: String) -> Arg;
    fn parse(&mut self, program: String);
}

impl Program {
    pub fn new() -> Self {
        Self { labels: vec![], tok: 0 }
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
            c = program.chars().nth(self.tok).unwrap();
        }

        result
    }

    fn skip_whitespace(&mut self, program: &String) {
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
        match arg.chars().nth(0).unwrap() {
            '.' => Arg::Label(arg[1..].to_string()),
            'r' => Arg::Reg(self.parse_register(arg)),
            '0' | '1' | '2' | '3' | '4' |
            '5' | '6' | '7' | '8' | '9' => 
                Arg::Imm(arg.parse::<u32>().unwrap()),
            _ => panic!("Wrong arg")
        }
    }

    // .main:
    //   mov r0, r1
    fn parse(&mut self, program: String) {
        while self.tok < program.len() {
            let c = program.chars().nth(self.tok).unwrap();
            if c == '.' {
                self.tok += 1;
                let label_name = self.skip_until(&program, ':');
                let mut instructions: Vec<Instruction> = Vec::new();

                loop {
                    self.tok += 1; self.skip_whitespace(&program);

                    let instruction_name = self.skip_until_whitespace(&program);
                    self.skip_whitespace(&program);

                    match instruction_name.as_str() {
                        "mov" => {
                            let arg1 = self.skip_until(&program, ',');
                            self.tok += 1; self.skip_whitespace(&program);
                            let arg2 = self.skip_until(&program, '\n');

                            let arg_dst = self.parse_arg(arg1);
                            let arg_src = self.parse_arg(arg2);

                            instructions.push(Instruction::Mov(arg_dst, arg_src));
                        },
                        _ => panic!("No such instruction")
                    }

                    if self.tok >= program.len() - 1 || program.chars().nth(self.tok).unwrap() == '.' {
                        break
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
            Instruction::Mov(dest, src) => write!(f, "Mov({}, {})", dest, src),
            _ => write!(f, "unknown()")
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