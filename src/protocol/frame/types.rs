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
    FDtf,
    FDtfDa,
    FDtfMa,
    FDtfFa,
    FDtfEnd,
    FSyn,
    FReSyn,
    FItf,
    //Ack
    FAckCreate,
    FAckSelect,
    FAckDeselect,
    FAckMsg,
    FAckOrf,
    FAckCrf,
    FAckRead,
    FAckWrite,
    FAckTransferEnd,
    FAckSyn,
    FAckReSyn,
    FAckIdt,
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
            // Start-End Transfer Phase
            [0xC0, 0x01, _, 0x00] => FrameType::FRead,
            [0xC0, 0x35, _, 0x00] => FrameType::FAckRead,
            [0xC0, 0x02, _, 0x00] => FrameType::FWrite,
            [0xC0, 0x36, _, 0x00] => FrameType::FAckWrite,
            [0xC0, 0x08, _, 0x00] => FrameType::FTransferEnd,
            [0xC0, 0x37, _, 0x00] => FrameType::FAckTransferEnd,
            // Data Transfer Phase
            [0x00, 0x00, _, 0x00] => FrameType::FDtf,
            [0x00, 0x41, _, 0x00] => FrameType::FDtfDa,
            [0x00, 0x40, _, 0x00] => FrameType::FDtfMa,
            [0x00, 0x42, _, 0x00] => FrameType::FDtfFa,
            [0xC0, 0x04, _, 0x00] => FrameType::FDtfEnd,
            [0xC0, 0x03, _, 0x00] => FrameType::FSyn,
            [0xC0, 0x38, _, 0x00] => FrameType::FAckSyn,
            [0xC0, 0x05, _, 0x00] => FrameType::FReSyn,
            [0xC0, 0x39, _, 0x00] => FrameType::FAckReSyn,
            // Interrupt Phase
            [0xC0, 0x06, _, 0x00] => FrameType::FItf,
            [0xC0, 0x3A, _, 0x00] => FrameType::FAckIdt,
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
            FrameType::FCreate => 0xC0,
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

    pub(crate) fn into_arr(self) -> [u8; 4] {
        match self {
            FrameType::FConnect => [0x40, 0x20, 0x00, 0x00],
            FrameType::FAConnect => [0x40, 0x21, 0x00, 0x00],
            FrameType::FRConnect => [0x40, 0x22, 0x00, 0x00],
            FrameType::FRelease => [0x40, 0x23, 0x00, 0x00],
            FrameType::FRelconf => [0x40, 0x24, 0x00, 0x00],
            FrameType::FAbort => [0x02, 0x25, 0x00, 0x00],
            FrameType::FCreate => [0xC0, 0x11, 0x00, 0x00],
            FrameType::FAckCreate => [0xC0, 0x30, 0x00, 0x00],
            FrameType::FSelect => [0xC0, 0x12, 0x00, 0x00],
            FrameType::FAckSelect => [0xC0, 0x31, 0x00, 0x00],
            FrameType::FDeselect => [0xC0, 0x13, 0x00, 0x00],
            FrameType::FAckDeselect => [0xC0, 0x32, 0x00, 0x00],
            FrameType::FMsg => [0xC0, 0x16, 0x00, 0x00],
            FrameType::FMsgDM => [0xC0, 0x17, 0x00, 0x00],
            FrameType::FMsgMM => [0xC0, 0x18, 0x00, 0x00],
            FrameType::FMsgFM => [0xC0, 0x19, 0x00, 0x00],
            FrameType::FAckMsg => [0xC0, 0x3B, 0x00, 0x00],
            FrameType::FOrf => [0xC0, 0x14, 0x00, 0x00],
            FrameType::FAckOrf => [0xC0, 0x33, 0x00, 0x00],
            FrameType::FCrf => [0xC0, 0x15, 0x00, 0x00],
            FrameType::FAckCrf => [0xC0, 0x34, 0x00, 0x00],
            FrameType::FRead => [0xC0, 0x01, 0x00, 0x00],
            FrameType::FAckRead => [0xC0, 0x35, 0x00, 0x00],
            FrameType::FWrite => [0xC0, 0x02, 0x00, 0x00],
            FrameType::FAckWrite => [0xC0, 0x36, 0x00, 0x00],
            FrameType::FTransferEnd => [0xC0, 0x08, 0x00, 0x00],
            FrameType::FAckTransferEnd => [0xC0, 0x37, 0x00, 0x00],
            FrameType::FDtf => [0x00, 0x00, 0x00, 0x00],
            FrameType::FDtfDa => [0x00, 0x41, 0x00, 0x00],
            FrameType::FDtfMa => [0x00, 0x40, 0x00, 0x00],
            FrameType::FDtfFa => [0x00, 0x42, 0x00, 0x00],
            FrameType::FDtfEnd => [0xC0, 0x04, 0x00, 0x00],
            FrameType::FSyn => [0xC0, 0x03, 0x00, 0x00],
            FrameType::FAckSyn => [0xC0, 0x38, 0x00, 0x00],
            FrameType::FReSyn => [0xC0, 0x05, 0x00, 0x00],
            FrameType::FAckReSyn => [0xC0, 0x39, 0x00, 0x00],
            FrameType::FItf => [0xC0, 0x06, 0x00, 0x00],
            FrameType::FAckIdt => [0xC0, 0x3A, 0x00, 0x00],
            FrameType::Unknown => unreachable!(),
            FrameType::FData => todo!(),
            FrameType::FCancel => todo!(),
            FrameType::FRestart => todo!(),
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
