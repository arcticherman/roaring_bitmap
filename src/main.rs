mod roaring_bitmap;
mod roaring_set;
mod vec_set;
// use roaring_bitmap::RoaringBitMap;
use roaring_set::RoaringSet;
/*
 task:
 把RoaringSet应用到RoaringBitMap里
 */
fn main() {

    // let mut roaring_bitmap = RoaringBitMap::new();
    // roaring_bitmap.add((1<<16)*2+1);
    let mut roaring_set = RoaringSet::new();
    for i in 1..32768{
        roaring_set.add(i);
    }
    for i in 8190..32768{
        roaring_set.remove(i);
    }
    println!("set.len():{} ", roaring_set.len());
    // println!("set.len():{} roaring_set:{:?}", roaring_set.len(), roaring_set.values());
}