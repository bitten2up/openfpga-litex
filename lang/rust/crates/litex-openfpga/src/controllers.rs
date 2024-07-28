use core::ops::BitAnd;

#[repr(u16)]
#[allow(dead_code)]
pub enum PocketControls {
    DpadUp = 1 << 0,
    DpadDown = 1 << 1,
    DpadLeft = 1 << 2,
    DpadRight = 1 << 3,
    FaceA = 1 << 4,
    FaceB = 1 << 5,
    FaceX = 1 << 6,
    FaceY = 1 << 7,
    TrigL1 = 1 << 8,
    TrigR1 = 1 << 9,
    TrigL2 = 1 << 10,
    TrigR2 = 1 << 11,
    TrigL3 = 1 << 12,
    TrigR3 = 1 << 13,
    FaceSelect = 1 << 14,
    FaceStart = 1 << 15,
}

impl BitAnd<PocketControls> for u16 {
    type Output = bool;

    fn bitand(self, rhs: PocketControls) -> Self::Output {
        (self & rhs as u16) > 0
    }
}
