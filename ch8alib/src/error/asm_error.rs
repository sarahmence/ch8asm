/*
 * asm_error.rs
 * Defines a union of assembler error types
 * Created on 12/16/2019
 * Created by Andrew Davis
 *
 * Copyright (C) 2019  Andrew Davis
 *
 * This program is free software: you can redistribute it and/or modify   
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 * 
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

//crate import
extern crate ch8_isa;

//usage statements
use super::LexerError;
use super::ParseError;
use super::OpcodeError;
use super::AddrError;
use super::ArgError;
use super::SkipError;
use ch8_isa::error::BinaryError;
use std::fmt;

/// An error generated when assembling a Chip-8 binary
pub enum AsmError {
    /// A lexer error 
    Lexer(LexerError),

    /// A parser error
    Parser(ParseError),

    /// An opcode generation error
    Opcode(OpcodeError),

    /// A binary generation error
    Binary(BinaryError),

    /// An address retrieval error
    Address(AddrError),

    /// An argument error
    Argument(ArgError),

    /// A skiptype error
    Skip(SkipError)
}

//Debug implementation
impl fmt::Debug for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AsmError::Lexer(ref le) => write!(f, "{:?}", le),
            AsmError::Parser(ref pe) => write!(f, "{:?}", pe),
            AsmError::Opcode(ref oe) => write!(f, "{:?}", oe),
            AsmError::Binary(ref be) => write!(f, "{:?}", be),
            AsmError::Address(ref ae) => write!(f, "{:?}", ae),
            AsmError::Argument(ref ae) => write!(f, "{:?}", ae),
            AsmError::Skip(ref se) => write!(f, "{:?}", se)
        }
    }
}

//Display implementation
impl fmt::Display for AsmError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AsmError::Lexer(ref le) => write!(f, "{}", le),
            AsmError::Parser(ref pe) => write!(f, "{}", pe),
            AsmError::Opcode(ref oe) => write!(f, "{}", oe),
            AsmError::Binary(ref be) => write!(f, "{}", be),
            AsmError::Address(ref ae) => write!(f, "{}", ae),
            AsmError::Argument(ref ae) => write!(f, "{}", ae),
            AsmError::Skip(ref se) => write!(f, "{}", se)
        }
    }
}

//end of file
