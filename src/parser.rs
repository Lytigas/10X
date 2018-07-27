use super::common::*;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Token {
    Move(Direction),
    Duplicate(Direction),
    Digit(u8),
    Undo,
    Kill,
    PlacePortal,
    JumpPortal,
    Inc,
    Dec,
    NoOp,
    Read,
    Write,
    Open,
    Close,
    Null,
}

use self::Token::*;

/*
Grammar:
Program ::= Instruction | Instruction Program | "[" Program "]"
Instruction ::= "," | "." | "|" | "-" | "+" | "j" | "o" | "k" | "u" | Duplicate | Move
Move ::= "M" Digit | "m" Digit
Duplicate ::= "X" Digit | "x" Digit
*/

pub fn parse(text: &str) -> Program {
    let tokens = tokenize(text);
    let mut ctx = Context::new(&tokens);
    parse_program(&mut ctx)
}

struct Context<'a> {
    tokens: &'a [Token],
    loop_depth: u32,
    index: usize,
}

impl<'a> Context<'a> {
    fn new(t: &'a [Token]) -> Self {
        Context {
            tokens: t,
            index: 0,
            loop_depth: 0,
        }
    }
    fn next(&mut self) -> &mut Self {
        self.index += 1;
        self
    }

    fn curr(&self) -> Token {
        self.tokens[self.index]
    }

    fn descend(&mut self) {
        self.loop_depth += 1;
    }

    fn asscend(&mut self) {
        assert!(self.loop_depth > 0, "Unmatched Loop Close");
        self.loop_depth -= 1;
    }
}

fn parse_program(ctx: &mut Context) -> Program {
    let mut program = Program::new();
    while ctx.index < ctx.tokens.len() {
        match ctx.curr() {
            Open => {
                ctx.descend();
                let body = parse_program(ctx.next());
                assert_eq!(ctx.curr(), Close, "Expected Close to end Loop.");
                program.push(Instruction::Loop(body));
                ctx.next();
            }
            Close => {
                ctx.asscend();
                return program;
            }
            _ => program.push(parse_instruction(ctx)),
        }
    }
    program
}

fn parse_instruction(ctx: &mut Context) -> Instruction {
    let i = match ctx.curr() {
        Undo => Instruction::Undo,
        Kill => Instruction::Kill,
        PlacePortal => Instruction::PlacePortal,
        JumpPortal => Instruction::JumpPortal,
        Inc => Instruction::Inc,
        Dec => Instruction::Dec,
        NoOp => Instruction::NoOp,
        Read => Instruction::Read,
        Write => Instruction::Write,
        Duplicate(ref _x) => parse_duplicate(ctx),
        Move(ref _x) => parse_move(ctx),
        _ => panic!(
            "Unexpected token when parsing instruction: {:?}",
            ctx.curr()
        ),
    };
    ctx.next();
    i
}

fn parse_duplicate(ctx: &mut Context) -> Instruction {
    match ctx.curr() {
        Duplicate(dir) => {
            ctx.next();
            match ctx.curr() {
                Digit(d) => Instruction::Duplicate(dir, Axis::from(d)),
                _ => panic!("Expected Digit after Duplicate"),
            }
        }
        _ => unreachable!(),
    }
}

fn parse_move(ctx: &mut Context) -> Instruction {
    match ctx.curr() {
        Move(dir) => {
            ctx.next();
            match ctx.curr() {
                Digit(d) => Instruction::Move(dir, Axis::from(d)),
                _ => panic!("Expected Digit after Move"),
            }
        }
        _ => unreachable!(),
    }
}

fn tokenize(program: &str) -> Vec<Token> {
    program
        .chars()
        .map(|c| match c {
            'M' => Move(Direction::Pos),
            'm' => Move(Direction::Neg),
            'X' => Duplicate(Direction::Pos),
            'x' => Duplicate(Direction::Neg),
            'u' => Undo,
            'k' => Kill,
            'o' => PlacePortal,
            'j' => JumpPortal,
            '+' => Inc,
            '-' => Dec,
            '|' => NoOp,
            '.' => Read,
            ',' => Write,
            '[' => Open,
            ']' => Close,
            '0'...'9' => Digit(c.to_digit(10).unwrap() as u8),
            _ => Null,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[should_panic]
    fn unmatched_close() {
        parse("[+-|[[++]M5X2]oj]m2]");
    }

    #[test]
    #[should_panic]
    fn unmatched_open() {
        parse("X4[+-|[[++]M5X2]ojm2");
    }

    #[test]
    fn integration() {
        let text = "M9m8x0X2ukoj[+-|.,]";
        let tokens = tokenize(text);
        assert_eq!(
            tokens,
            vec![
                Token::Move(Direction::Pos),
                Token::Digit(9),
                Token::Move(Direction::Neg),
                Token::Digit(8),
                Token::Duplicate(Direction::Neg),
                Token::Digit(0),
                Token::Duplicate(Direction::Pos),
                Token::Digit(2),
                Token::Undo,
                Token::Kill,
                Token::PlacePortal,
                Token::JumpPortal,
                Token::Open,
                Token::Inc,
                Token::Dec,
                Token::NoOp,
                Token::Read,
                Token::Write,
                Token::Close,
            ]
        );

        let mut ctx = Context::new(&tokens);
        let program = parse_program(&mut ctx);
        assert_eq!(
            program,
            vec![
                Instruction::Move(Direction::Pos, Axis::from(9)),
                Instruction::Move(Direction::Neg, Axis::from(8)),
                Instruction::Duplicate(Direction::Neg, Axis::from(0)),
                Instruction::Duplicate(Direction::Pos, Axis::from(2)),
                Instruction::Undo,
                Instruction::Kill,
                Instruction::PlacePortal,
                Instruction::JumpPortal,
                Instruction::Loop(vec![
                    Instruction::Inc,
                    Instruction::Dec,
                    Instruction::NoOp,
                    Instruction::Read,
                    Instruction::Write,
                ]),
            ]
        );
    }
}
