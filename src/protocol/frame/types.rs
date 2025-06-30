#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum FrameType {
    #[default]
    Unknown,
    FConnect,
    FAConnect,
    FRConnect,
    FRelease,
    FRelconf,
    FAbort,
    FCreate,
    FSelect,
    FDeselect,
    FOrf,
    FCrf,
    FRead,
    FWrite,
    FData,
    FTransferEnd,
    FCancel,
    FRestart,
    FMsg,
    FMsgDM,
    FMsgMM,
    FMsgFM,

    //Ack
    FAckCreate,
    FAckSelect,
    FAckDeselect,
    FAckMsg,
    FAckOrf,
    FAckCrf,
}

impl FrameType {
    pub fn from_header(value: [u8; 4]) -> Self {
        log::debug!("Parsing frame type from header: {value:x?}");
        match value {
            // Connect Phase
            [0x40, 0x20, 0x00, _] => FrameType::FConnect,
            [0x40, 0x21, _, _] => FrameType::FAConnect,
            [0x40, 0x22, _, _] => FrameType::FRConnect,
            [0x40, 0x23, _, _] => FrameType::FRelease,
            [0x40, 0x24, _, _] => FrameType::FRelconf,
            [0x02, 0x25, _, _] => FrameType::FAbort,
            // Select Phase
            [0xC0, 0x11, _, 0x00] => FrameType::FCreate,
            [0xC0, 0x30, _, 0x00] => FrameType::FAckCreate,
            [0xC0, 0x12, _, 0x00] => FrameType::FSelect,
            [0xC0, 0x31, _, 0x00] => FrameType::FAckSelect,
            [0xC0, 0x13, _, 0x00] => FrameType::FDeselect,
            [0xC0, 0x32, _, 0x00] => FrameType::FAckDeselect,
            [0xC0, 0x16, _, 0x00] => FrameType::FMsg,
            [0xC0, 0x17, _, 0x00] => FrameType::FMsgDM,
            [0xC0, 0x18, _, 0x00] => FrameType::FMsgMM,
            [0xC0, 0x19, _, 0x00] => FrameType::FMsgFM,
            [0xC0, 0x3B, _, 0x00] => FrameType::FAckMsg,
            // Open Phase
            [0xC0, 0x14, _, 0x00] => FrameType::FOrf,
            [0xC0, 0x33, _, 0x00] => FrameType::FAckOrf,
            [0xC0, 0x15, _, 0x00] => FrameType::FCrf,
            [0xC0, 0x34, _, 0x00] => FrameType::FAckCrf,
            _ => todo!("not implemented frame type: {value:x?}"),
        }
    }

    fn into_u8(self) -> u8 {
        match self {
            FrameType::FConnect
            | FrameType::FAConnect
            | FrameType::FRConnect
            | FrameType::FRelease
            | FrameType::FRelconf => 0x40,
            FrameType::FAbort => todo!(),
            FrameType::FCreate => todo!(),
            FrameType::FSelect => todo!(),
            FrameType::FOrf => todo!(),
            FrameType::FCrf => todo!(),
            FrameType::FRead => todo!(),
            FrameType::FWrite => todo!(),
            FrameType::FData => todo!(),
            FrameType::FTransferEnd => todo!(),
            FrameType::FCancel => todo!(),
            FrameType::FRestart => todo!(),
            FrameType::FMsg => todo!(),
            FrameType::Unknown => 0x00,
            _ => todo!("not implemented frame type: {self:?}"),
        }
    }
}

impl From<[u8; 4]> for FrameType {
    fn from(value: [u8; 4]) -> Self {
        FrameType::from_header(value)
    }
}

impl From<FrameType> for u8 {
    fn from(val: FrameType) -> Self {
        val.into_u8()
    }
}
