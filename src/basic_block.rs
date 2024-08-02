use std::ptr::null_mut;

use llvm_sys::{core::LLVMGetBasicBlockTerminator, prelude::LLVMBasicBlockRef};

use crate::value::Value;

#[derive(Debug, Clone)]
pub struct BasicBlock(LLVMBasicBlockRef);

impl BasicBlock {
    /// Create a new [`BasicBlock`] from a [`LLVMBasicBlockRef`].
    pub(crate) fn new(pointer: LLVMBasicBlockRef) -> BasicBlock {
        assert_ne!(pointer, null_mut(), "basic block pointer is null");
        BasicBlock(pointer)
    }

    #[inline]
    /// Get inner [`LLVMBasicBlockRef`].
    pub(crate) fn get(&self) -> LLVMBasicBlockRef {
        self.0
    }

    /// Get the block terminator [`Value`].
    pub fn get_block_terminator(&self) -> Option<Value> {
        let value = unsafe { LLVMGetBasicBlockTerminator(self.get()) };

        if value == null_mut() {
            return None;
        }

        Some(Value::new(value))
    }
}
