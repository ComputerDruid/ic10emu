use std::collections::HashMap;

use ic10emu::grammar::{Identifier, InstructionOp, Number, Operand};
use strum::EnumProperty;

fn main() {
    let [_, filename] = &std::env::args().collect::<Vec<_>>()[..] else {
        panic!("need just a filename");
    };
    let text = std::fs::read_to_string(filename).unwrap();
    let mut program = ic10emu::interpreter::Program::try_from_code(&text).expect("valid ic10 code");
    let mut substitutions = HashMap::<Identifier, Operand>::new();
    for (line, i) in program.instructions.iter_mut().enumerate() {
        for operand in i.operands.iter_mut() {
            if let Operand::Identifier(ident) = operand {
                if let Some(alias) = substitutions.get(ident) {
                    *operand = alias.clone()
                }
            }
        }
        match i.instruction {
            InstructionOp::Alias => {
                let [ident, alias] = &i.operands[..] else {
                    panic!("bad alias line {line}")
                };
                let Operand::Identifier(ident) = ident else {
                    panic!("bad alias ident line {line}")
                };
                substitutions.insert(ident.clone(), alias.clone());
                match alias {
                    Operand::RegisterSpec { .. } => {
                        i.instruction = InstructionOp::Nop;
                        i.operands = vec![];
                    }
                    Operand::DeviceSpec { .. } => {
                        //keep device aliases for screwdriver operation
                    }
                    _ => panic!("bad alias type {alias:?} on line {line}"),
                }
            }
            InstructionOp::Define => {
                let [ident, sub] = &i.operands[..] else {
                    panic!("bad define line {line}")
                };
                let Operand::Identifier(ident) = ident else {
                    panic!("bad define ident line {line}")
                };
                substitutions.insert(ident.clone(), sub.clone());
                i.instruction = InstructionOp::Nop;
                i.operands = vec![];
            }
            _ => {}
        }
    }
    let mut line_mapping = HashMap::<u32, u32>::new();
    {
        let mut old_range = 0..=0;
        let mut new_line = 0;
        for (old_line, i) in program.instructions.iter().enumerate() {
            old_range = *old_range.start()..=old_line;
            if i.instruction != InstructionOp::Nop {
                for old_line in old_range {
                    line_mapping.insert(old_line.try_into().unwrap(), new_line);
                }
                new_line += 1;
                old_range = old_line + 1..=old_line + 1;
            }
        }
        for old_line in old_range {
            line_mapping.insert(old_line.try_into().unwrap(), new_line);
        }
    }
    for i in program.instructions.iter_mut() {
        for operand in i.operands.iter_mut() {
            match operand {
                Operand::Identifier(ident) => {
                    if let Some(label) = program.labels.get(&ident.name) {
                        let &new_line = line_mapping
                            .get(label)
                            .expect("label should have been inbounds...");
                        *operand = Operand::Number(ic10emu::grammar::Number::Float(new_line.into()))
                    }
                }
                Operand::Number(n @ Number::String(_)) => {
                    let value = n.value();
                    *n = Number::Float(value);
                }
                Operand::LogicType(lt) => {
                    if let Some(value) = lt
                        .get_str("value")
                        .map(|val| val.parse::<u8>().unwrap() as f64)
                    {
                        *operand = Operand::Number(Number::Float(value))
                    }
                }
                Operand::BatchMode(bm) => {
                    if let Some(value) = bm
                        .get_str("value")
                        .map(|val| val.parse::<u8>().unwrap() as f64)
                    {
                        *operand = Operand::Number(Number::Float(value))
                    }
                }
                Operand::SlotLogicType(slt) => {
                    if let Some(value) = slt
                        .get_str("value")
                        .map(|val| val.parse::<u8>().unwrap() as f64)
                    {
                        *operand = Operand::Number(Number::Float(value))
                    }
                }
                Operand::ReagentMode(rm) => {
                    if let Some(value) = rm
                        .get_str("value")
                        .map(|val| val.parse::<u8>().unwrap() as f64)
                    {
                        *operand = Operand::Number(Number::Float(value))
                    }
                }
                _ => {}
            }
        }
    }
    for (_new_line, i) in program
        .instructions
        .iter()
        .filter(|i| i.instruction != InstructionOp::Nop)
        .enumerate()
    {
        println!("{i}");
    }
}
