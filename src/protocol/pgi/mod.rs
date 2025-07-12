use crate::protocol::pi::{
    pi11::Pi11, pi12::Pi12, pi3::Pi3, pi31::Pi31, pi32::Pi32, pi33::Pi33, pi34::Pi34, pi36::Pi36,
    pi37::Pi37, pi4::Pi4, pi41::Pi41, pi42::Pi42, pi51::Pi51, pi52::Pi52,
};

pub(crate) enum Pgi {
    FileDescriptor(Pi3, Pi4, Pi11, Pi12),
    LogicalAttr(Pi31, Pi32, Pi33, Pi34, Pi36, Pi37 /* Pi38, Pi39 */),
    PhysicalAttr(Pi41, Pi42),
    HistoricAttr(Pi51, Pi52),
}
