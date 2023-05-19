use serde::Deserialize;

bitflags::bitflags!(
    #[derive(Deserialize, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct Test: i32 {
        const FLAG1 = 0x01;
        const FLAG2 = 0x02;
        const FLAG3 = 0x04;
        const FLAG4 = 0x08;
    }
);

fn main() {
}
