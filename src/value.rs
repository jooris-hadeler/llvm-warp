use std::ptr::null_mut;

use llvm_sys::{
    analysis::{LLVMVerifierFailureAction, LLVMVerifyFunction},
    core::{LLVMGetParam, LLVMSetLinkage, LLVMTypeOf},
    prelude::*,
    LLVMIntPredicate, LLVMLinkage, LLVMRealPredicate,
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

/// Predicate for Integer comparisions.
#[derive(Debug, Clone, Copy)]
pub enum IntPredicate {
    Eq,
    Ne,
    UGt,
    UGe,
    ULt,
    ULe,
    SGt,
    SGe,
    SLt,
    SLe,
}

impl From<IntPredicate> for LLVMIntPredicate {
    fn from(value: IntPredicate) -> Self {
        match value {
            IntPredicate::Eq => Self::LLVMIntEQ,
            IntPredicate::Ne => Self::LLVMIntNE,
            IntPredicate::UGt => Self::LLVMIntUGT,
            IntPredicate::UGe => Self::LLVMIntUGE,
            IntPredicate::ULt => Self::LLVMIntULT,
            IntPredicate::ULe => Self::LLVMIntULE,
            IntPredicate::SGt => Self::LLVMIntSGT,
            IntPredicate::SGe => Self::LLVMIntSGE,
            IntPredicate::SLt => Self::LLVMIntSLT,
            IntPredicate::SLe => Self::LLVMIntSLE,
        }
    }
}

/// Predicate for Real comparisions.
#[derive(Debug, Clone, Copy)]
pub enum RealPredicate {
    PredicateFalse,
    OEq,
    OGt,
    OGe,
    OLt,
    OLe,
    ONe,
    ORd,
    UNo,
    UEq,
    UGt,
    UGe,
    ULt,
    ULe,
    UNe,
    PredicateTrue,
}

impl From<RealPredicate> for LLVMRealPredicate {
    fn from(value: RealPredicate) -> Self {
        match value {
            RealPredicate::PredicateFalse => Self::LLVMRealPredicateFalse,
            RealPredicate::OEq => Self::LLVMRealOEQ,
            RealPredicate::OGt => Self::LLVMRealOGT,
            RealPredicate::OGe => Self::LLVMRealOGE,
            RealPredicate::OLt => Self::LLVMRealOLT,
            RealPredicate::OLe => Self::LLVMRealOLE,
            RealPredicate::ONe => Self::LLVMRealONE,
            RealPredicate::ORd => Self::LLVMRealORD,
            RealPredicate::UNo => Self::LLVMRealUNO,
            RealPredicate::UEq => Self::LLVMRealUEQ,
            RealPredicate::UGt => Self::LLVMRealUGT,
            RealPredicate::UGe => Self::LLVMRealUGE,
            RealPredicate::ULt => Self::LLVMRealULT,
            RealPredicate::ULe => Self::LLVMRealULE,
            RealPredicate::UNe => Self::LLVMRealUNE,
            RealPredicate::PredicateTrue => Self::LLVMRealPredicateTrue,
        }
    }
}
