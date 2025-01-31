use bitvec::{array::BitArray, order::Lsb0};

fn main() {
    dbg!(BitArray::<[u64; 4], Lsb0>::new([0u64; 4]));
}
