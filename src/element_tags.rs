use bitflags::bitflags;

bitflags! {
    #[derive(Debug, Clone, PartialEq)]
    pub struct ElementTags: u32 {
        const StatusBar = 1 << 0;
        const WinBar = 1 << 1;
        const FloatBorder = 1 << 2;
        const FloatTitle = 1 << 3;
        const Top = 1 << 4;
        const Bottom = 1 << 5;
        const Left = 1 << 6;
        const Right = 1 << 7;
    }
}
