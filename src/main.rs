use bitvec::{array::BitArray, order::Lsb0};

fn main() {
    let a = BitArray::<_, Lsb0>::new([0u64; 4]);
    dbg!(a);
}
