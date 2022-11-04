mod roaring_bitmap;
mod roaring_set;
// use roaring_bitmap::RoaringBitMap;
// use std::collections::BTreeSet;
use roaring_set::RoaringSet;
fn main() {
    // let mut roaring_bitmap = RoaringBitMap::new();
    // roaring_bitmap.add((1<<16)*2+1);
    // println!("tree_set.len():{}", tree_set.len());
    let mut set = RoaringSet::new();
    set.add(1);
    set.add(2);
    set.add(2);
    set.add(3);
    set.remove(2);
    println!("set.len():{}", set.len());
}