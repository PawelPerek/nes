use enum_as_inner::EnumAsInner;

#[derive(EnumAsInner)]
pub enum InstructionArguments {
    Zero,
    One(u8),
    Two(u8, u8)
}