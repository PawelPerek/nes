use enum_as_inner::EnumAsInner;

#[derive(Clone, Default, EnumAsInner)]
pub enum InstructionArguments {
    #[default] Zero,
    One(u8),
    Two(u8, u8)
}