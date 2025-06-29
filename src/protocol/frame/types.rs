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
    FOpen,
    FClose,
    FRead,
    FWrite,
    FData,
    FTransferEnd,
    FCancel,
    FRestart,
    FMessage,
}

impl FrameType {
    pub fn from_header(value: [u8; 4]) -> Self {
        log::debug!("Parsing frame type from header: {value:x?}");
        match value {
            [0x40, 0x20, 0x00, _] => FrameType::FConnect,
            [0x40, 0x21, _, _] => FrameType::FAConnect,
            [0x40, 0x22, _, _] => FrameType::FRConnect,
            [0x40, 0x23, _, _] => FrameType::FRelease,
            [0x40, 0x24, _, _] => FrameType::FRelconf,
            [0x02, 0x25, _, _] => FrameType::FAbort,
            [0x03, 0x00, 0x00, 0x00] => FrameType::FCreate,
            [0x04, 0x00, 0x00, 0x00] => FrameType::FSelect,
            [0x05, 0x00, 0x00, 0x00] => FrameType::FOpen,
            [0x06, 0x00, 0x00, 0x00] => FrameType::FClose,
            [0x07, 0x00, 0x00, 0x00] => FrameType::FRead,
            [0x08, 0x00, 0x00, 0x00] => FrameType::FWrite,
            [0x09, 0x00, 0x00, 0x00] => FrameType::FData,
            [0x0A, 0x00, 0x00, 0x00] => FrameType::FTransferEnd,
            [0x0B, 0x00, 0x00, 0x00] => FrameType::FCancel,
            [0x0C, 0x00, 0x00, 0x00] => FrameType::FRestart,
            [0x0D, 0x00, 0x00, 0x00] => FrameType::FMessage,
            _ => FrameType::Unknown,
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
            FrameType::FOpen => todo!(),
            FrameType::FClose => todo!(),
            FrameType::FRead => todo!(),
            FrameType::FWrite => todo!(),
            FrameType::FData => todo!(),
            FrameType::FTransferEnd => todo!(),
            FrameType::FCancel => todo!(),
            FrameType::FRestart => todo!(),
            FrameType::FMessage => todo!(),
            FrameType::Unknown => 0x00,
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
