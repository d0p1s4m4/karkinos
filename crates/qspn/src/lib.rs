use bitflags::bitflags;

bitflags! {
    pub struct Flags: u32 {
        const MAP_ME = 1;
        const MAP_VOID = 1 << 1;
        const MAP_HNODE = 1 << 2;
        const MAP_BNODE = 1 << 3;
        const MAP_ERNODE = 1 << 4;
        const MAP_GNODE = 1 << 5;
        const MAP_RNODE = 1 << 6;
        const MAP_UPDATE = 1 << 7;
        const QSPN_CLOSED = 1 << 8;
        const QSPN_OPENED = 1 << 9;
        const QSPN_OLD = 1 << 10;
        const QSPN_STARTER = 1 << 11;
        const QSPN_OPENER = 1 << 12;
        const MAP_IGW = 1 << 13;
    }
}

pub const MAX_GROUP_NODE: u16 = 1 << 8;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub fn open() {}

pub fn close() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
