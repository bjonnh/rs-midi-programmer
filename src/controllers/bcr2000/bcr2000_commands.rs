pub enum BCR2000Commands {
    Id,
    IdReply,
    BCL,
    BCLReply,
    PresetSelection,
    FirmwareSend,
    FirmwareReply,
    RequestData,
    RequestSetup,
    RequestPresetName,
    RequestSnapshot,
    SendText,
}

impl BCR2000Commands {
    pub(crate) fn to_u8(&self) -> u8 {
        match self {
            BCR2000Commands::Id => 0x01,
            BCR2000Commands::IdReply => 0x02,
            BCR2000Commands::BCL => 0x20,
            BCR2000Commands::BCLReply => 0x21,
            BCR2000Commands::PresetSelection => 0x22,
            BCR2000Commands::FirmwareSend => 0x34,
            BCR2000Commands::FirmwareReply => 0x35,
            BCR2000Commands::RequestData => 0x40,
            BCR2000Commands::RequestSetup => 0x41,
            BCR2000Commands::RequestPresetName => 0x42,
            BCR2000Commands::RequestSnapshot => 0x43,
            BCR2000Commands::SendText => 0x78,
        }
    }
}
