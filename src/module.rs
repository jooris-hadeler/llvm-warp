use std::ptr::null_mut;

use llvm_sys::{
    bit_writer::LLVMWriteBitcodeToFile,
    core::{LLVMAddFunction, LLVMDisposeModule, LLVMSetTarget},
    prelude::LLVMModuleRef,
    target_machine::{LLVMCodeGenFileType, LLVMCodeGenOptLevel, LLVMCodeModel, LLVMRelocMode},
};

use crate::{
    get_default_target_triple, initialize_all_asm_parsers, initialize_all_asm_printers, initialize_all_target_infos, initialize_all_target_mcs, initialize_all_targets, target::{Target, TargetMachine}, ty::Type, util::string_to_cstring, value::Value
};

#[derive(Debug, Clone)]
pub struct Module(LLVMModuleRef);

impl Module {
    /// Creates a new [`Module`] from a [`LLVMModuleRef`].
    pub(crate) fn new(pointer: LLVMModuleRef) -> Self {
        assert_ne!(pointer, null_mut(), "module pointer is null");
        Self(pointer)
    }

    /// Get the inner [`LLVMModuleRef`].
    pub(crate) fn get(&self) -> LLVMModuleRef {
        self.0
    }

    /// Set module target.
    pub fn set_target(&self, target_triple: String) {
        let target = string_to_cstring(target_triple);

        unsafe { LLVMSetTarget(self.get(), target.as_ptr()) }
    }

    /// Add a new function to this [`Module`].
    pub fn add_function<S: ToString>(&self, name: S, func_ty: &Type) -> Value {
        let name = string_to_cstring(name.to_string());

        unsafe { Value::new(LLVMAddFunction(self.get(), name.as_ptr(), func_ty.get())) }
    }

    /// Write bitcode to file.
    pub fn write_bitcode_to_file<S: ToString>(&self, path: S) -> bool {
        let path = string_to_cstring(path.to_string());

        unsafe { LLVMWriteBitcodeToFile(self.get(), path.as_ptr()) == 1 }
    }

    /// Write object file.
    pub fn write_object_file(
        self,
        target_triple: Option<String>,
        file: String,
    ) -> Result<(), String> {
        initialize_all_target_infos();
        initialize_all_targets();
        initialize_all_target_mcs();
        initialize_all_asm_parsers();
        initialize_all_asm_printers();

        let target_triple = target_triple.unwrap_or_else(|| get_default_target_triple());

        let target = Target::get_target_from_triple(target_triple.as_str())?;

        let target_machine = TargetMachine::create(
            target,
            target_triple.as_str(),
            "",
            "",
            LLVMCodeGenOptLevel::LLVMCodeGenLevelDefault,
            LLVMRelocMode::LLVMRelocPIC,
            LLVMCodeModel::LLVMCodeModelDefault,
        );

        target_machine.emit_to_file(self, file, LLVMCodeGenFileType::LLVMObjectFile)
    }

    /// Dispose of this [`Module`] and free its memory.
    pub fn dispose(self) {
        unsafe { LLVMDisposeModule(self.get()) }
    }
}
