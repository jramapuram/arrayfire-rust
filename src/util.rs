use defines::AfError;
use defines::Aftype;
use defines::InterpType;
use defines::ConvMode;
use defines::ConvDomain;
use defines::MatProp;
use defines::MatchType;
use defines::ColorMap;
use std::mem;

impl From<i32> for AfError {
    fn from(t: i32) -> AfError {
        assert!(AfError::SUCCESS as i32 <= t && t <= AfError::ERR_UNKNOWN as i32);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for Aftype {
    fn from(t: u8) -> Aftype {
        assert!(Aftype::F32 as u8 <= t && t <= Aftype::U64 as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for InterpType {
    fn from(t: u8) -> InterpType {
        assert!(InterpType::NEAREST as u8 <= t && t <= InterpType::CUBIC as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for ConvMode {
    fn from(t: u8) -> ConvMode {
        assert!(ConvMode::DEFAULT as u8 <= t && t <= ConvMode::EXPAND as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for ConvDomain {
    fn from(t: u8) -> ConvDomain {
        assert!(ConvDomain::AUTO as u8 <= t && t <= ConvDomain::FREQUENCY as u8);
        unsafe { mem::transmute(t) }
    }
}

impl From<u8> for MatchType {
    fn from(t: u8) -> MatchType {
        assert!(MatchType::SAD as u8 <= t && t <= MatchType::SHD as u8);
        unsafe { mem::transmute(t) }
    }
}

pub fn to_u32(t: MatProp) -> u32 {
    match t {
        MatProp::NONE       =>  0,
        MatProp::TRANS      =>  1,
        MatProp::CTRANS     =>  2,
        MatProp::UPPER      =>  32,
        MatProp::LOWER      =>  64,
        MatProp::DIAGUNIT  =>  128,
        MatProp::SYM        =>  512,
        MatProp::POSDEF     =>  1024,
        MatProp::ORTHOG     =>  2048,
        MatProp::TRIDIAG   =>  4096,
        MatProp::BLOCKDIAG =>  8192,
    }
}

impl From<i32> for ColorMap {
    fn from(t: i32) -> ColorMap {
        assert!(ColorMap::DEFAULT as i32 <= t && t <= ColorMap::BLUE as i32);
        unsafe { mem::transmute(t) }
    }
}
