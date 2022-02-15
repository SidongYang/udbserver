use crate::reg::RegMap;
use unicorn_engine::RegisterARM64;

pub static REGMAP: RegMap = RegMap {
    regs: &[
        (Some(RegisterARM64::X0 as i32), 8),
        (Some(RegisterARM64::X1 as i32), 8),
        (Some(RegisterARM64::X2 as i32), 8),
        (Some(RegisterARM64::X3 as i32), 8),
        (Some(RegisterARM64::X4 as i32), 8),
        (Some(RegisterARM64::X5 as i32), 8),
        (Some(RegisterARM64::X6 as i32), 8),
        (Some(RegisterARM64::X7 as i32), 8),
        (Some(RegisterARM64::X8 as i32), 8),
        (Some(RegisterARM64::X9 as i32), 8),
        (Some(RegisterARM64::X10 as i32), 8),
        (Some(RegisterARM64::X11 as i32), 8),
        (Some(RegisterARM64::X12 as i32), 8),
        (Some(RegisterARM64::X13 as i32), 8),
        (Some(RegisterARM64::X14 as i32), 8),
        (Some(RegisterARM64::X15 as i32), 8),
        (Some(RegisterARM64::X16 as i32), 8),
        (Some(RegisterARM64::X17 as i32), 8),
        (Some(RegisterARM64::X18 as i32), 8),
        (Some(RegisterARM64::X19 as i32), 8),
        (Some(RegisterARM64::X20 as i32), 8),
        (Some(RegisterARM64::X21 as i32), 8),
        (Some(RegisterARM64::X22 as i32), 8),
        (Some(RegisterARM64::X23 as i32), 8),
        (Some(RegisterARM64::X24 as i32), 8),
        (Some(RegisterARM64::X25 as i32), 8),
        (Some(RegisterARM64::X26 as i32), 8),
        (Some(RegisterARM64::X27 as i32), 8),
        (Some(RegisterARM64::X28 as i32), 8),
        (Some(RegisterARM64::X29 as i32), 8),
        (Some(RegisterARM64::X30 as i32), 8),
        (Some(RegisterARM64::SP as i32), 8),
        (Some(RegisterARM64::PC as i32), 8),
        (None, 4), // cpsr
        (Some(RegisterARM64::V0 as i32), 16),
        (Some(RegisterARM64::V1 as i32), 16),
        (Some(RegisterARM64::V2 as i32), 16),
        (Some(RegisterARM64::V3 as i32), 16),
        (Some(RegisterARM64::V4 as i32), 16),
        (Some(RegisterARM64::V5 as i32), 16),
        (Some(RegisterARM64::V6 as i32), 16),
        (Some(RegisterARM64::V7 as i32), 16),
        (Some(RegisterARM64::V8 as i32), 16),
        (Some(RegisterARM64::V9 as i32), 16),
        (Some(RegisterARM64::V10 as i32), 16),
        (Some(RegisterARM64::V11 as i32), 16),
        (Some(RegisterARM64::V12 as i32), 16),
        (Some(RegisterARM64::V13 as i32), 16),
        (Some(RegisterARM64::V14 as i32), 16),
        (Some(RegisterARM64::V15 as i32), 16),
        (Some(RegisterARM64::V16 as i32), 16),
        (Some(RegisterARM64::V17 as i32), 16),
        (Some(RegisterARM64::V18 as i32), 16),
        (Some(RegisterARM64::V19 as i32), 16),
        (Some(RegisterARM64::V20 as i32), 16),
        (Some(RegisterARM64::V21 as i32), 16),
        (Some(RegisterARM64::V22 as i32), 16),
        (Some(RegisterARM64::V23 as i32), 16),
        (Some(RegisterARM64::V24 as i32), 16),
        (Some(RegisterARM64::V25 as i32), 16),
        (Some(RegisterARM64::V26 as i32), 16),
        (Some(RegisterARM64::V27 as i32), 16),
        (Some(RegisterARM64::V28 as i32), 16),
        (Some(RegisterARM64::V29 as i32), 16),
        (Some(RegisterARM64::V30 as i32), 16),
        (Some(RegisterARM64::V31 as i32), 16),
        (None, 4), // fpsr
        (None, 4), // fpcr
    ],
    len: 33,
    desc: r#"<target version="1.0"><architecture>aarch64</architecture></target>"#,
};