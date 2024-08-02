pub mod basic_block;
pub mod builder;
pub mod context;
pub mod module;
pub mod ty;
mod util;
pub mod value;

use std::ffi::CString;

pub use llvm_sys;
use llvm_sys::{
    target::{
        LLVM_InitializeAllAsmParsers, LLVM_InitializeAllAsmPrinters, LLVM_InitializeAllTargetInfos,
        LLVM_InitializeAllTargetMCs, LLVM_InitializeAllTargets,
    },
    target_machine::LLVMGetDefaultTargetTriple,
};
use util::cstring_to_string;

#[cfg(test)]
mod test;

pub fn get_default_target_triple() -> String {
    let target_triple = unsafe { CString::from_raw(LLVMGetDefaultTargetTriple()) };
    cstring_to_string(target_triple)
}

pub fn initialize_all_target_infos() {
    unsafe { LLVM_InitializeAllTargetInfos() };
}

pub fn initialize_all_targets() {
    unsafe { LLVM_InitializeAllTargets() };
}

pub fn initialize_all_target_mcs() {
    unsafe { LLVM_InitializeAllTargetMCs() };
}

pub fn initialize_all_asm_parsers() {
    unsafe { LLVM_InitializeAllAsmParsers() };
}

pub fn initialize_all_asm_printers() {
    unsafe { LLVM_InitializeAllAsmPrinters() };
}
