// GENERATED CODE DO NOT MODIFY
#[derive(PartialEq, Debug, IntoStaticStr, AsRefStr)]
pub enum InstructionOp {
     #[rust_sitter::leaf(text = "sne" )]Sne,
     #[rust_sitter::leaf(text = "bna" )]Bna,
     #[rust_sitter::leaf(text = "get" )]Get,
     #[rust_sitter::leaf(text = "lbns" )]Lbns,
     #[rust_sitter::leaf(text = "poke" )]Poke,
     #[rust_sitter::leaf(text = "rand" )]Rand,
     #[rust_sitter::leaf(text = "srl" )]Srl,
     #[rust_sitter::leaf(text = "acos" )]Acos,
     #[rust_sitter::leaf(text = "sna" )]Sna,
     #[rust_sitter::leaf(text = "beqal" )]Beqal,
     #[rust_sitter::leaf(text = "brgez" )]Brgez,
     #[rust_sitter::leaf(text = "sgtz" )]Sgtz,
     #[rust_sitter::leaf(text = "blt" )]Blt,
     #[rust_sitter::leaf(text = "div" )]Div,
     #[rust_sitter::leaf(text = "sle" )]Sle,
     #[rust_sitter::leaf(text = "sap" )]Sap,
     #[rust_sitter::leaf(text = "move" )]Move,
     #[rust_sitter::leaf(text = "sltz" )]Sltz,
     #[rust_sitter::leaf(text = "bltzal" )]Bltzal,
     #[rust_sitter::leaf(text = "sdns" )]Sdns,
     #[rust_sitter::leaf(text = "slez" )]Slez,
     #[rust_sitter::leaf(text = "ld" )]Ld,
     #[rust_sitter::leaf(text = "sll" )]Sll,
     #[rust_sitter::leaf(text = "ble" )]Ble,
     #[rust_sitter::leaf(text = "sb" )]Sb,
     #[rust_sitter::leaf(text = "l" )]L,
     #[rust_sitter::leaf(text = "brdse" )]Brdse,
     #[rust_sitter::leaf(text = "add" )]Add,
     #[rust_sitter::leaf(text = "seq" )]Seq,
     #[rust_sitter::leaf(text = "getd" )]Getd,
     #[rust_sitter::leaf(text = "bap" )]Bap,
     #[rust_sitter::leaf(text = "ceil" )]Ceil,
     #[rust_sitter::leaf(text = "sra" )]Sra,
     #[rust_sitter::leaf(text = "lbs" )]Lbs,
     #[rust_sitter::leaf(text = "sd" )]Sd,
     #[rust_sitter::leaf(text = "bapz" )]Bapz,
     #[rust_sitter::leaf(text = "snez" )]Snez,
     #[rust_sitter::leaf(text = "bgezal" )]Bgezal,
     #[rust_sitter::leaf(text = "log" )]Log,
     #[rust_sitter::leaf(text = "and" )]And,
     #[rust_sitter::leaf(text = "nor" )]Nor,
     #[rust_sitter::leaf(text = "bgtal" )]Bgtal,
     #[rust_sitter::leaf(text = "sleep" )]Sleep,
     #[rust_sitter::leaf(text = "beqzal" )]Beqzal,
     #[rust_sitter::leaf(text = "bnan" )]Bnan,
     #[rust_sitter::leaf(text = "put" )]Put,
     #[rust_sitter::leaf(text = "sbn" )]Sbn,
     #[rust_sitter::leaf(text = "seqz" )]Seqz,
     #[rust_sitter::leaf(text = "sapz" )]Sapz,
     #[rust_sitter::leaf(text = "blez" )]Blez,
     #[rust_sitter::leaf(text = "bnez" )]Bnez,
     #[rust_sitter::leaf(text = "jal" )]Jal,
     #[rust_sitter::leaf(text = "snan" )]Snan,
     #[rust_sitter::leaf(text = "snanz" )]Snanz,
     #[rust_sitter::leaf(text = "xor" )]Xor,
     #[rust_sitter::leaf(text = "brap" )]Brap,
     #[rust_sitter::leaf(text = "brgtz" )]Brgtz,
     #[rust_sitter::leaf(text = "bnazal" )]Bnazal,
     #[rust_sitter::leaf(text = "select" )]Select,
     #[rust_sitter::leaf(text = "bneal" )]Bneal,
     #[rust_sitter::leaf(text = "sgt" )]Sgt,
     #[rust_sitter::leaf(text = "slt" )]Slt,
     #[rust_sitter::leaf(text = "brnaz" )]Brnaz,
     #[rust_sitter::leaf(text = "bge" )]Bge,
     #[rust_sitter::leaf(text = "push" )]Push,
     #[rust_sitter::leaf(text = "ss" )]Ss,
     #[rust_sitter::leaf(text = "abs" )]Abs,
     #[rust_sitter::leaf(text = "atan" )]Atan,
     #[rust_sitter::leaf(text = "asin" )]Asin,
     #[rust_sitter::leaf(text = "beq" )]Beq,
     #[rust_sitter::leaf(text = "beqz" )]Beqz,
     #[rust_sitter::leaf(text = "brlez" )]Brlez,
     #[rust_sitter::leaf(text = "hcf" )]Hcf,
     #[rust_sitter::leaf(text = "sgez" )]Sgez,
     #[rust_sitter::leaf(text = "bne" )]Bne,
     #[rust_sitter::leaf(text = "atan2" )]Atan2,
     #[rust_sitter::leaf(text = "bltal" )]Bltal,
     #[rust_sitter::leaf(text = "bnaal" )]Bnaal,
     #[rust_sitter::leaf(text = "cos" )]Cos,
     #[rust_sitter::leaf(text = "lbn" )]Lbn,
     #[rust_sitter::leaf(text = "brapz" )]Brapz,
     #[rust_sitter::leaf(text = "bnaz" )]Bnaz,
     #[rust_sitter::leaf(text = "brltz" )]Brltz,
     #[rust_sitter::leaf(text = "pop" )]Pop,
     #[rust_sitter::leaf(text = "bdnsal" )]Bdnsal,
     #[rust_sitter::leaf(text = "brlt" )]Brlt,
     #[rust_sitter::leaf(text = "sla" )]Sla,
     #[rust_sitter::leaf(text = "sdse" )]Sdse,
     #[rust_sitter::leaf(text = "brgt" )]Brgt,
     #[rust_sitter::leaf(text = "bgtz" )]Bgtz,
     #[rust_sitter::leaf(text = "bnezal" )]Bnezal,
     #[rust_sitter::leaf(text = "breq" )]Breq,
     #[rust_sitter::leaf(text = "lr" )]Lr,
     #[rust_sitter::leaf(text = "snaz" )]Snaz,
     #[rust_sitter::leaf(text = "bdseal" )]Bdseal,
     #[rust_sitter::leaf(text = "alias" )]Alias,
     #[rust_sitter::leaf(text = "bleal" )]Bleal,
     #[rust_sitter::leaf(text = "floor" )]Floor,
     #[rust_sitter::leaf(text = "round" )]Round,
     #[rust_sitter::leaf(text = "exp" )]Exp,
     #[rust_sitter::leaf(text = "brna" )]Brna,
     #[rust_sitter::leaf(text = "trunc" )]Trunc,
     #[rust_sitter::leaf(text = "ls" )]Ls,
     #[rust_sitter::leaf(text = "putd" )]Putd,
     #[rust_sitter::leaf(text = "brnan" )]Brnan,
     #[rust_sitter::leaf(text = "bgeal" )]Bgeal,
     #[rust_sitter::leaf(text = "sin" )]Sin,
     #[rust_sitter::leaf(text = "bapal" )]Bapal,
     #[rust_sitter::leaf(text = "sqrt" )]Sqrt,
     #[rust_sitter::leaf(text = "sub" )]Sub,
     #[rust_sitter::leaf(text = "brne" )]Brne,
     #[rust_sitter::leaf(text = "define" )]Define,
     #[rust_sitter::leaf(text = "max" )]Max,
     #[rust_sitter::leaf(text = "label" )]Label,
     #[rust_sitter::leaf(text = "mul" )]Mul,
     #[rust_sitter::leaf(text = "brge" )]Brge,
     #[rust_sitter::leaf(text = "min" )]Min,
     #[rust_sitter::leaf(text = "brdns" )]Brdns,
     #[rust_sitter::leaf(text = "mod" )]Mod,
     #[rust_sitter::leaf(text = "j" )]J,
     #[rust_sitter::leaf(text = "or" )]Or,
     #[rust_sitter::leaf(text = "bdns" )]Bdns,
     #[rust_sitter::leaf(text = "breqz" )]Breqz,
     #[rust_sitter::leaf(text = "brle" )]Brle,
     #[rust_sitter::leaf(text = "sbs" )]Sbs,
     #[rust_sitter::leaf(text = "jr" )]Jr,
     #[rust_sitter::leaf(text = "blezal" )]Blezal,
     #[rust_sitter::leaf(text = "bapzal" )]Bapzal,
     #[rust_sitter::leaf(text = "s" )]S,
     #[rust_sitter::leaf(text = "sge" )]Sge,
     #[rust_sitter::leaf(text = "bgtzal" )]Bgtzal,
     #[rust_sitter::leaf(text = "lb" )]Lb,
     #[rust_sitter::leaf(text = "peek" )]Peek,
     #[rust_sitter::leaf(text = "brnez" )]Brnez,
     #[rust_sitter::leaf(text = "bgez" )]Bgez,
     #[rust_sitter::leaf(text = "not" )]Not,
     #[rust_sitter::leaf(text = "yield" )]Yield,
     #[rust_sitter::leaf(text = "bdse" )]Bdse,
     #[rust_sitter::leaf(text = "bgt" )]Bgt,
     #[rust_sitter::leaf(text = "tan" )]Tan,
     #[rust_sitter::leaf(text = "bltz" )]Bltz,
}
