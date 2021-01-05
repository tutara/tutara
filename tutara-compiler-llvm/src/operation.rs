use inkwell::values::{FloatValue, InstructionValue, IntValue};

pub enum Operation<'a> {
	FloatValue(FloatValue<'a>),
	BoolValue(IntValue<'a>),
	Return(InstructionValue<'a>),
	NoOp,
}
