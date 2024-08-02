use std::ptr::null_mut;

use llvm_sys::{
    core::{
        LLVMBuildAShr, LLVMBuildAdd, LLVMBuildAlloca, LLVMBuildAnd, LLVMBuildBitCast, LLVMBuildBr,
        LLVMBuildCall2, LLVMBuildCast, LLVMBuildCondBr, LLVMBuildFAdd, LLVMBuildFCmp,
        LLVMBuildFDiv, LLVMBuildFMul, LLVMBuildFNeg, LLVMBuildFPCast, LLVMBuildFPExt,
        LLVMBuildFPToSI, LLVMBuildFPToUI, LLVMBuildFPTrunc, LLVMBuildFRem, LLVMBuildFSub,
        LLVMBuildICmp, LLVMBuildLoad2, LLVMBuildMul, LLVMBuildNeg, LLVMBuildNot, LLVMBuildOr,
        LLVMBuildRet, LLVMBuildRetVoid, LLVMBuildSDiv, LLVMBuildSExt, LLVMBuildSIToFP,
        LLVMBuildSRem, LLVMBuildStore, LLVMBuildSub, LLVMBuildUDiv, LLVMBuildUIToFP, LLVMBuildURem,
        LLVMBuildZExt, LLVMConstInt, LLVMDisposeBuilder, LLVMGetInsertBlock, LLVMPositionBuilder,
        LLVMPositionBuilderAtEnd, LLVMPositionBuilderBefore,
    },
    prelude::{LLVMBuilderRef, LLVMValueRef},
    LLVMIntPredicate, LLVMOpcode, LLVMRealPredicate,
};

use crate::{
    basic_block::BasicBlock,
    ty::Type,
    util::{string_to_cstring, EMPTY_TWINE},
    value::Value,
};

#[derive(Debug, Clone)]
pub struct Builder(LLVMBuilderRef);

impl Builder {
    /// Create a new [`Builder`] from a [`LLVMBuilderRef`].
    pub(crate) fn new(pointer: LLVMBuilderRef) -> Self {
        assert_ne!(pointer, null_mut(), "builder pointer is null");
        Self(pointer)
    }

    #[inline]
    /// Get inner [`LLVMBuilderRef`].
    pub(crate) fn get(&self) -> LLVMBuilderRef {
        self.0
    }

    /// Positions the builder before the given Instruction/[`Value`].
    pub fn position(&self, block: &BasicBlock, instr: &Value) {
        unsafe { LLVMPositionBuilder(self.get(), block.get(), instr.get()) }
    }

    /// Position the builder before the given Instruction/[`Value`] in a given [`BasicBlock`].
    pub fn position_before(&self, instr: &Value) {
        unsafe { LLVMPositionBuilderBefore(self.get(), instr.get()) }
    }

    /// Position the builder at the end of a [`BasicBlockRef`].
    pub fn position_at_end(&self, block: &BasicBlock) {
        unsafe { LLVMPositionBuilderAtEnd(self.get(), block.get()) }
    }

    /// Get insertion block.
    pub fn get_insert_block(&self) -> BasicBlock {
        unsafe { BasicBlock::new(LLVMGetInsertBlock(self.get())) }
    }

    /// Build an `Add` instruction.
    pub fn build_add(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildAdd(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `Sub` instruction.
    pub fn build_sub(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildSub(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `Mul` instruction.
    pub fn build_mul(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildMul(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `SDiv` instruction.
    pub fn build_sdiv(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildSDiv(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `UDiv` instruction.
    pub fn build_udiv(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildUDiv(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `SRem` instruction.
    pub fn build_srem(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildSRem(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `URem` instruction.
    pub fn build_urem(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildURem(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build an `ICmp` instruction.
    pub fn build_icmp(&self, op: LLVMIntPredicate, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildICmp(
                self.get(),
                op,
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build an `ArithmeticShiftRight` instruction.
    pub fn build_ashr(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildAShr(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build an `And` instruction.
    pub fn build_and(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildAnd(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `BitCast` instruction.
    pub fn build_bit_cast(&self, ty: &Type, value: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildBitCast(
                self.get(),
                value.get(),
                ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FAdd` instruction.
    pub fn build_fadd(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildFAdd(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FCmp` instruction.
    pub fn build_fcmp(&self, predicate: LLVMRealPredicate, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildFCmp(
                self.get(),
                predicate,
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FDiv` instruction.
    pub fn build_fdiv(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildFDiv(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FMul` instruction.
    pub fn build_fmul(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildFMul(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FNeg` instruction.
    pub fn build_fneg(&self, value: &Value) -> Value {
        unsafe { Value::new(LLVMBuildFNeg(self.get(), value.get(), EMPTY_TWINE.as_ptr())) }
    }

    /// Build a `FPCast` instruction.
    pub fn build_fpcast(&self, value: &Value, ty: &Type) -> Value {
        unsafe {
            Value::new(LLVMBuildFPCast(
                self.get(),
                value.get(),
                ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FPExt` instruction.
    pub fn build_fpext(&self, value: &Value, ty: &Type) -> Value {
        unsafe {
            Value::new(LLVMBuildFPExt(
                self.get(),
                value.get(),
                ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FPToSI` instruction.
    pub fn build_fptosi(&self, value: &Value, ty: &Type) -> Value {
        unsafe {
            Value::new(LLVMBuildFPToSI(
                self.get(),
                value.get(),
                ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FPToUI` instruction.
    pub fn build_fptoui(&self, value: &Value, ty: &Type) -> Value {
        unsafe {
            Value::new(LLVMBuildFPToUI(
                self.get(),
                value.get(),
                ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FPTrunc` instruction.
    pub fn build_fptrunc(&self, value: &Value, ty: &Type) -> Value {
        unsafe {
            Value::new(LLVMBuildFPTrunc(
                self.get(),
                value.get(),
                ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FRem` instruction.
    pub fn build_frem(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildFRem(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `FSub` instruction.
    pub fn build_fsub(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildFSub(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build an `Or` instruction.
    pub fn build_or(&self, left: &Value, right: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildOr(
                self.get(),
                left.get(),
                right.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `SItoFP` instruction.
    pub fn build_sitofp(&self, value: &Value, ty: &Type) -> Value {
        unsafe {
            Value::new(LLVMBuildSIToFP(
                self.get(),
                value.get(),
                ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `UItoFP` instruction.
    pub fn build_uitofp(&self, value: &Value, ty: &Type) -> Value {
        unsafe {
            Value::new(LLVMBuildUIToFP(
                self.get(),
                value.get(),
                ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `Cast` instruction.
    pub fn build_cast(&self, op: LLVMOpcode, value: &Value, ty: &Type) -> Value {
        unsafe {
            Value::new(LLVMBuildCast(
                self.get(),
                op,
                value.get(),
                ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `Neg` instruction.
    pub fn build_neg(&self, value: &Value) -> Value {
        unsafe { Value::new(LLVMBuildNeg(self.get(), value.get(), EMPTY_TWINE.as_ptr())) }
    }

    /// Build a `Not` isntruction.
    pub fn build_not(&self, value: &Value) -> Value {
        unsafe { Value::new(LLVMBuildNot(self.get(), value.get(), EMPTY_TWINE.as_ptr())) }
    }

    /// Build an `Alloca` instruction.
    pub fn build_alloca<S: ToString>(&self, ty: &Type, twine: S) -> Value {
        let twine = string_to_cstring(twine.to_string());

        unsafe { Value::new(LLVMBuildAlloca(self.get(), ty.get(), twine.as_ptr())) }
    }

    /// Build a `Load` instruction.
    pub fn build_load(&self, ty: &Type, pointer: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildLoad2(
                self.get(),
                ty.get(),
                pointer.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `Store` instruction.
    pub fn build_store(&self, value: &Value, pointer: &Value) -> Value {
        unsafe { Value::new(LLVMBuildStore(self.get(), value.get(), pointer.get())) }
    }

    /// Build a `ReturnVoid` instruction.
    pub fn build_return_void(&self) -> Value {
        unsafe { Value::new(LLVMBuildRetVoid(self.get())) }
    }

    /// Build a `Return` instruction.
    pub fn build_return(&self, value: &Value) -> Value {
        unsafe { Value::new(LLVMBuildRet(self.get(), value.get())) }
    }

    /// Build a `SignExtension` instruction.
    pub fn build_sext(&self, dest_ty: &Type, value: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildSExt(
                self.get(),
                value.get(),
                dest_ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `ZeroExtension` instruction.
    pub fn build_zext(&self, dest_ty: &Type, value: &Value) -> Value {
        unsafe {
            Value::new(LLVMBuildZExt(
                self.get(),
                value.get(),
                dest_ty.get(),
                EMPTY_TWINE.as_ptr(),
            ))
        }
    }

    /// Build a `CondBranch` instruction.
    pub fn build_cond_br(
        &self,
        condition: &Value,
        then_block: &BasicBlock,
        else_block: &BasicBlock,
    ) {
        unsafe {
            LLVMBuildCondBr(
                self.get(),
                condition.get(),
                then_block.get(),
                else_block.get(),
            )
        };
    }

    /// Build a `Branch` instruction.
    pub fn build_br(&self, block: &BasicBlock) {
        unsafe { LLVMBuildBr(self.get(), block.get()) };
    }

    /// Build a `Call` instruction.
    pub fn build_call<S: ToString>(
        &self,
        func_ty: &Type,
        func: &Value,
        arguments: &[Value],
        twine: S,
    ) -> Value {
        let twine = string_to_cstring(twine.to_string());

        let mut arguments = arguments
            .iter()
            .map(|arg| arg.get())
            .collect::<Vec<LLVMValueRef>>();

        unsafe {
            Value::new(LLVMBuildCall2(
                self.get(),
                func_ty.get(),
                func.get(),
                arguments.as_mut_ptr(),
                arguments.len() as u32,
                twine.as_ptr(),
            ))
        }
    }

    /// Create a const integer.
    pub fn const_int(&self, integer_ty: &Type, value: u64, sign_extend: bool) -> Value {
        unsafe { Value::new(LLVMConstInt(integer_ty.get(), value, sign_extend as i32)) }
    }

    /// Dispose the builder and free its memory.
    pub fn dispose(self) {
        unsafe { LLVMDisposeBuilder(self.get()) }
    }
}
