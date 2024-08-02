use std::{ffi::CString, ptr::null_mut};

use llvm_sys::{
    target::LLVMTargetDataRef,
    target_machine::{
        LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMCreateTargetDataLayout, LLVMCreateTargetMachine, LLVMGetTargetFromName, LLVMGetTargetFromTriple, LLVMRelocMode, LLVMTargetMachineEmitToFile, LLVMTargetMachineRef, LLVMTargetRef
    },
};

use crate::{module::Module, util::{cstring_to_string, string_to_cstring}};

#[derive(Debug, Clone, Copy)]
pub struct TargetData(LLVMTargetDataRef);

impl TargetData {
    pub fn get(&self) -> LLVMTargetDataRef {
        self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Target(LLVMTargetRef);

impl Target {
    pub fn get(&self) -> LLVMTargetRef {
        self.0
    }

    pub fn get_target_from_name<S: ToString>(name: S) -> Self {
        let name = string_to_cstring(name.to_string());
        unsafe { Target(LLVMGetTargetFromName(name.as_ptr())) }
    }

    pub fn get_target_from_triple<S: ToString>(triple: S) -> Result<Self, String> {
        let triple = string_to_cstring(triple.to_string());
        let mut target: LLVMTargetRef = null_mut();
        let error_message = null_mut();

        let found =
            unsafe { LLVMGetTargetFromTriple(triple.as_ptr(), &mut target, error_message) == 0 };

        if found {
            return Ok(Target(target));
        }

        let error_message = unsafe { CString::from_raw(*error_message) };
        Err(cstring_to_string(error_message))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TargetMachine(LLVMTargetMachineRef);

impl TargetMachine {
    pub fn get(&self) -> LLVMTargetMachineRef {
        self.0
    }

    pub fn create<S: ToString, C: ToString, F: ToString>(
        target: Target,
        triple: S,
        cpu: C,
        features: F,
        level: LLVMCodeGenOptLevel,
        reloc: LLVMRelocMode,
        code_model: LLVMCodeModel,
    ) -> Self {
        let triple = string_to_cstring(triple.to_string());
        let cpu = string_to_cstring(cpu.to_string());
        let features = string_to_cstring(features.to_string());

        unsafe {
            TargetMachine(LLVMCreateTargetMachine(
                target.get(),
                triple.as_ptr(),
                cpu.as_ptr(),
                features.as_ptr(),
                level,
                reloc,
                code_model,
            ))
        }
    }

    pub fn create_data_layout(&self) -> TargetData {
        unsafe { TargetData(LLVMCreateTargetDataLayout(self.get())) }
    }

    pub fn emit_to_file<S: ToString>(
        &self,
        module: Module,
        file: S,
        file_type: LLVMCodeGenFileType,
    ) -> Result<(), String> {
        let file = CString::new(file.to_string()).expect("failed to convert String to CString");
        let mut error = null_mut();

        let errored = unsafe {
            LLVMTargetMachineEmitToFile(
                self.get(),
                module.get(),
                file.as_ptr() as *mut _,
                file_type,
                &mut error,
            ) == 1
        };

        if !errored {
            return Ok(());
        }

        let error_message = unsafe { CString::from_raw(error) };
        let error_message = error_message
            .to_str()
            .expect("failed to convert CString to String");

        Err(error_message.to_string())
    }
}
