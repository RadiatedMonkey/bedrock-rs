/// Bitflags for AvailableCommand's ParameterDataEntry's options
#[allow(proto_gen)]
pub enum CommandParameterOption {
    None = 0,
    EnumAutocompleteExpansion = 0x01,
    HasSemanticConstraint = 0x02,
    EnumAsChainedCommand = 0x04,
}