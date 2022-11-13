mod roaring_bitmap;
mod roaring_bitmap64;
mod roaring_set;
mod vec_set;
pub use roaring_bitmap64::RoaringBitMap64;
pub use roaring_bitmap::RoaringBitMap;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut roaring_bitmap = RoaringBitMap::new();
        roaring_bitmap.add(1);
        roaring_bitmap.add(1);
        roaring_bitmap.add(2);
        roaring_bitmap.add(3);
        roaring_bitmap.add(4);
        roaring_bitmap.add(4294967293);
        roaring_bitmap.add(4294967294);
        roaring_bitmap.add(4294967295);
        roaring_bitmap.remove(4);
        println!("roaring_bitmap: {:?} ", roaring_bitmap.values());
    
        let mut roaring_bitmap64 = RoaringBitMap64::new();
        roaring_bitmap64.add(1);
        roaring_bitmap64.add(1);
        roaring_bitmap64.add(2);
        roaring_bitmap64.add(3);
        roaring_bitmap64.add(4);
        roaring_bitmap64.add(18446744073709551613);
        roaring_bitmap64.add(18446744073709551614);
        roaring_bitmap64.add(18446744073709551615);
        roaring_bitmap64.remove(4);
        println!("roaring_bitmap64: {:?} ", roaring_bitmap64.values());
    }
}