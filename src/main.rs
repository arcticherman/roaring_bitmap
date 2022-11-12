mod roaring_bitmap;
mod roaring_bitmap64;
mod roaring_set;
mod vec_set;
use roaring_bitmap64::RoaringBitMap64;
// use roaring_set::RoaringSet;
fn main() {
    let mut roaring_bitmap = RoaringBitMap64::new();
    // for i in 1..32768{
    //     roaring_bitmap.add(i);
    // }
    // for i in 8190..32768{
    //     roaring_bitmap.remove(i);
    // }
    // println!("roaring_bitmap.len() :{} ", roaring_bitmap.len());

    roaring_bitmap.add(1);
    roaring_bitmap.add(2);
    roaring_bitmap.add(65534);
    roaring_bitmap.add(65535);
    roaring_bitmap.add(18446744073709551613);
    roaring_bitmap.add(18446744073709551614);
    roaring_bitmap.add(18446744073709551615);
    println!("roaring_bitmap :{:?} ", roaring_bitmap.values());
}