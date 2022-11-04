mod roaring_bitmap;
use roaring_bitmap::RoaringBitMap;
use std::collections::BTreeSet;
use roaring_set;
fn main() {
    let mut roaring_bitmap = RoaringBitMap::new();
    roaring_bitmap.add((1<<16)*2+1);
    let mut tree_set:BTreeSet<u16> = BTreeSet::new();
    tree_set.insert(1);
    tree_set.insert(2);
    tree_set.insert(2);
    println!("tree_set.len():{}", tree_set.len());
}