/*
 * mod.rs
 * Module header for the error module for ch8asm
 * Created on 12/8/2019
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

//exports
mod lexer_error;
pub use lexer_error::LexerError;
mod addr_error;
pub use addr_error::AddrError;
mod variant_error;
pub use variant_error::VariantError;
mod parse_error;
pub use parse_error::ParseError;
mod asm_error;
pub use asm_error::AsmError;
mod opcode_error;
pub use opcode_error::OpcodeError;
mod arg_error;
pub use arg_error::ArgError;
mod skip_error;
pub use skip_error::SkipError;

//end of file
