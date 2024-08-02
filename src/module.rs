use std::ptr::null_mut;

use llvm_sys::{
    bit_writer::LLVMWriteBitcodeToFile,
    core::{LLVMAddFunction, LLVMDisposeModule},
    prelude::LLVMModuleRef,
};

use crate::{ty::Type, util::string_to_cstring, value::Value};

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

    /// Dispose of this [`Module`] and free its memory.
    pub fn dispose(self) {
        unsafe { LLVMDisposeModule(self.get()) }
    }
}
