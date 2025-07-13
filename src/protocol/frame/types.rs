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
            [0xC0, 0x30, _, _] => FrameType::FAckCreate,
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

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! test_header {
        ($header: tt, $excepted: path) => {
            let header = $header;
            assert_eq!(FrameType::from_header(header), $excepted);
        };
    }

    #[test]
    fn test_connect_phase() {
        test_header!([0x40, 0x20, 0x00, 0x01], FrameType::FConnect);
        test_header!([0x40, 0x21, 0xAB, 0xCD], FrameType::FAConnect);
        test_header!([0x40, 0x22, 0xAB, 0xCD], FrameType::FRConnect);
        test_header!([0x40, 0x23, 0xAB, 0xCD], FrameType::FRelease);
        test_header!([0x40, 0x24, 0xAB, 0xCD], FrameType::FRelconf);
        test_header!([0x02, 0x25, 0xAB, 0xCD], FrameType::FAbort);
    }
    #[test]
    fn test_select_phase() {
        test_header!([0xC0, 0x11, 0x00, 0x00], FrameType::FCreate);
        test_header!([0xC0, 0x30, 0x12, 0x34], FrameType::FAckCreate);
        test_header!([0xC0, 0x12, 0x00, 0x00], FrameType::FSelect);
        test_header!([0xC0, 0x31, 0x00, 0x00], FrameType::FAckSelect);
        test_header!([0xC0, 0x13, 0x00, 0x00], FrameType::FDeselect);
        test_header!([0xC0, 0x32, 0x00, 0x00], FrameType::FAckDeselect);
        test_header!([0xC0, 0x16, 0x00, 0x00], FrameType::FMsg);
        test_header!([0xC0, 0x17, 0x00, 0x00], FrameType::FMsgDM);
        test_header!([0xC0, 0x18, 0x00, 0x00], FrameType::FMsgMM);
        test_header!([0xC0, 0x19, 0x00, 0x00], FrameType::FMsgFM);
        test_header!([0xC0, 0x3B, 0x00, 0x00], FrameType::FAckMsg);
    }

    #[test]
    fn test_open_phase() {
        test_header!([0xC0, 0x14, 0x00, 0x00], FrameType::FOrf);
        test_header!([0xC0, 0x33, 0x00, 0x00], FrameType::FAckOrf);
        test_header!([0xC0, 0x15, 0x00, 0x00], FrameType::FCrf);
        test_header!([0xC0, 0x34, 0x00, 0x00], FrameType::FAckCrf);
    }

    #[test]
    fn test_startend_phase() {
        test_header!([0xC0, 0x01, 0x00, 0x00], FrameType::FRead);
        test_header!([0xC0, 0x35, 0x00, 0x00], FrameType::FAckRead);
        test_header!([0xC0, 0x02, 0x00, 0x00], FrameType::FWrite);
        test_header!([0xC0, 0x36, 0x00, 0x00], FrameType::FAckWrite);
        test_header!([0xC0, 0x08, 0x00, 0x00], FrameType::FTransferEnd);
        test_header!([0xC0, 0x37, 0x00, 0x00], FrameType::FAckTransferEnd);
    }

    #[test]
    fn test_data_transfer_phase() {
        test_header!([0x00, 0x00, 0x12, 0x00], FrameType::FDtf);
        test_header!([0x00, 0x41, 0x56, 0x00], FrameType::FDtfDa);
        test_header!([0x00, 0x40, 0x78, 0x00], FrameType::FDtfMa);
        test_header!([0x00, 0x42, 0x9A, 0x00], FrameType::FDtfFa);
        test_header!([0xC0, 0x04, 0x00, 0x00], FrameType::FDtfEnd);
        test_header!([0xC0, 0x03, 0x00, 0x00], FrameType::FSyn);
        test_header!([0xC0, 0x38, 0x00, 0x00], FrameType::FAckSyn);
        test_header!([0xC0, 0x05, 0x00, 0x00], FrameType::FReSyn);
        test_header!([0xC0, 0x39, 0x00, 0x00], FrameType::FAckReSyn);
    }

    #[test]
    fn test_interrupt_phase() {
        test_header!([0xC0, 0x06, 0x00, 0x00], FrameType::FItf);
        test_header!([0xC0, 0x3A, 0x00, 0x00], FrameType::FAckIdt);
    }

    #[test]
    fn test_unknown_frame_type_panics() {
        let header = [0xFF, 0xFF, 0xFF, 0xFF];

        let result = std::panic::catch_unwind(|| {
            FrameType::from_header(header);
        });

        assert!(result.is_err(), "Expected panic for unknown frame type");
    }

    #[test]
    fn test_into_u8_connect_variants() {
        assert_eq!(FrameType::FConnect.into_u8(), 0x40);
        assert_eq!(FrameType::FAConnect.into_u8(), 0x40);
        assert_eq!(FrameType::FRConnect.into_u8(), 0x40);
        assert_eq!(FrameType::FRelease.into_u8(), 0x40);
        assert_eq!(FrameType::FRelconf.into_u8(), 0x40);
    }

    #[test]
    fn test_into_u8_create_variant() {
        assert_eq!(FrameType::FCreate.into_u8(), 0xC0);
    }

    #[test]
    fn test_into_u8_unknown_variant() {
        assert_eq!(FrameType::Unknown.into_u8(), 0x00);
    }

    #[test]
    fn test_frame_type_into_u8_conversion() {
        let value: u8 = FrameType::FCreate.into();
        assert_eq!(value, 0xC0);
    }

    #[test]
    fn test_into_arr_connect_variants() {
        assert_eq!(FrameType::FConnect.into_arr(), [0x40, 0x20, 0x00, 0x00]);
        assert_eq!(FrameType::FAConnect.into_arr(), [0x40, 0x21, 0x00, 0x00]);
        assert_eq!(FrameType::FRConnect.into_arr(), [0x40, 0x22, 0x00, 0x00]);
        assert_eq!(FrameType::FRelease.into_arr(), [0x40, 0x23, 0x00, 0x00]);
        assert_eq!(FrameType::FRelconf.into_arr(), [0x40, 0x24, 0x00, 0x00]);
    }

    #[test]
    fn test_into_arr_abort_and_create_variants() {
        assert_eq!(FrameType::FAbort.into_arr(), [0x02, 0x25, 0x00, 0x00]);
        assert_eq!(FrameType::FCreate.into_arr(), [0xC0, 0x11, 0x00, 0x00]);
        assert_eq!(FrameType::FAckCreate.into_arr(), [0xC0, 0x30, 0x00, 0x00]);
    }

    #[test]
    fn test_into_arr_select_variants() {
        assert_eq!(FrameType::FSelect.into_arr(), [0xC0, 0x12, 0x00, 0x00]);
        assert_eq!(FrameType::FAckSelect.into_arr(), [0xC0, 0x31, 0x00, 0x00]);
    }

    #[test]
    fn test_into_arr_deselect_variants() {
        assert_eq!(FrameType::FDeselect.into_arr(), [0xC0, 0x13, 0x00, 0x00]);
        assert_eq!(FrameType::FAckDeselect.into_arr(), [0xC0, 0x32, 0x00, 0x00]);
    }

    #[test]
    fn test_into_arr_msg_variants() {
        assert_eq!(FrameType::FMsg.into_arr(), [0xC0, 0x16, 0x00, 0x00]);
        assert_eq!(FrameType::FMsgDM.into_arr(), [0xC0, 0x17, 0x00, 0x00]);
        assert_eq!(FrameType::FMsgMM.into_arr(), [0xC0, 0x18, 0x00, 0x00]);
        assert_eq!(FrameType::FMsgFM.into_arr(), [0xC0, 0x19, 0x00, 0x00]);
        assert_eq!(FrameType::FAckMsg.into_arr(), [0xC0, 0x3B, 0x00, 0x00]);
    }

    #[test]
    fn test_into_arr_orf_crf_variants() {
        assert_eq!(FrameType::FOrf.into_arr(), [0xC0, 0x14, 0x00, 0x00]);
        assert_eq!(FrameType::FAckOrf.into_arr(), [0xC0, 0x33, 0x00, 0x00]);
        assert_eq!(FrameType::FCrf.into_arr(), [0xC0, 0x15, 0x00, 0x00]);
        assert_eq!(FrameType::FAckCrf.into_arr(), [0xC0, 0x34, 0x00, 0x00]);
    }

    #[test]
    fn test_into_arr_read_write_variants() {
        assert_eq!(FrameType::FRead.into_arr(), [0xC0, 0x01, 0x00, 0x00]);
        assert_eq!(FrameType::FAckRead.into_arr(), [0xC0, 0x35, 0x00, 0x00]);
        assert_eq!(FrameType::FWrite.into_arr(), [0xC0, 0x02, 0x00, 0x00]);
        assert_eq!(FrameType::FAckWrite.into_arr(), [0xC0, 0x36, 0x00, 0x00]);
    }

    #[test]
    fn test_into_arr_transfer_end_variants() {
        assert_eq!(FrameType::FTransferEnd.into_arr(), [0xC0, 0x08, 0x00, 0x00]);
        assert_eq!(
            FrameType::FAckTransferEnd.into_arr(),
            [0xC0, 0x37, 0x00, 0x00]
        );
    }

    #[test]
    fn test_into_arr_dtf_variants() {
        assert_eq!(FrameType::FDtf.into_arr(), [0x00, 0x00, 0x00, 0x00]);
        assert_eq!(FrameType::FDtfDa.into_arr(), [0x00, 0x41, 0x00, 0x00]);
        assert_eq!(FrameType::FDtfMa.into_arr(), [0x00, 0x40, 0x00, 0x00]);
        assert_eq!(FrameType::FDtfFa.into_arr(), [0x00, 0x42, 0x00, 0x00]);
        assert_eq!(FrameType::FDtfEnd.into_arr(), [0xC0, 0x04, 0x00, 0x00]);
    }

    #[test]
    fn test_into_arr_sync_variants() {
        assert_eq!(FrameType::FSyn.into_arr(), [0xC0, 0x03, 0x00, 0x00]);
        assert_eq!(FrameType::FAckSyn.into_arr(), [0xC0, 0x38, 0x00, 0x00]);
        assert_eq!(FrameType::FReSyn.into_arr(), [0xC0, 0x05, 0x00, 0x00]);
        assert_eq!(FrameType::FAckReSyn.into_arr(), [0xC0, 0x39, 0x00, 0x00]);
    }

    #[test]
    fn test_into_arr_itf_variants() {
        assert_eq!(FrameType::FItf.into_arr(), [0xC0, 0x06, 0x00, 0x00]);
        assert_eq!(FrameType::FAckIdt.into_arr(), [0xC0, 0x3A, 0x00, 0x00]);
    }
}
