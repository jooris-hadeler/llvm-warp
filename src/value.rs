use std::ptr::null_mut;

use llvm_sys::{
    analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction},
    core::{LLVMDeleteFunction, LLVMGetParam, LLVMSetLinkage, LLVMTypeOf},
    prelude::*,
    LLVMLinkage,
};

use crate::ty::Type;

#[derive(Debug, Clone)]
pub struct Value(pub(crate) LLVMValueRef);

impl Value {
    /// Create a new [`Value`] from a [`LLVMValueRef`].
    pub(crate) fn new(pointer: LLVMValueRef) -> Value {
        assert_ne!(pointer, null_mut(), "value pointer is null");
        Self(pointer)
    }

    #[inline]
    /// Get inner [`LLVMValueRef`].
    pub(crate) fn get(&self) -> LLVMValueRef {
        self.0
    }

    /// Get function parameter [`Value`].
    pub fn get_param(&self, index: usize) -> Value {
        unsafe { Value::new(LLVMGetParam(self.get(), index as u32)) }
    }

    /// Set the linkage of a Function.
    pub fn set_linkage(&self, linkage: LLVMLinkage) {
        unsafe { LLVMSetLinkage(self.get(), linkage) }
    }

    /// Get [`Type`] of the current [`Value`].
    pub fn get_type(&self) -> Type {
        unsafe { Type::new(LLVMTypeOf(self.get())) }
    }

    /// Verify a function, returns `true` if the function is valid.
    pub fn verify_function(&self, action: VerifierFailureAction) -> bool {
        unsafe { LLVMVerifyFunction(self.get(), action.into()) != 1 }
    }

    /// Delete this function.
    pub fn delete_function(&self) {
        unsafe { LLVMDeleteFunction(self.get()) };
    }
}

/// A list of actions if a verification fails.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum VerifierFailureAction {
    AbortProcess,
    PrintMessage,
    ReturnStatus,
}

impl From<VerifierFailureAction> for LLVMVerifierFailureAction {
    fn from(value: VerifierFailureAction) -> Self {
        match value {
            VerifierFailureAction::AbortProcess => Self::LLVMAbortProcessAction,
            VerifierFailureAction::PrintMessage => Self::LLVMPrintMessageAction,
            VerifierFailureAction::ReturnStatus => Self::LLVMReturnStatusAction,
        }
    }
}
