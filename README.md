# Roaring Bitmap
Roaring Bitmap data structure
# Example
  let mut roaring_bitmap = RoaringBitMap::new();  
  roaring_bitmap.add(1);  
  roaring_bitmap.add(1);  
  roaring_bitmap.add(2);  
  roaring_bitmap.add(3);  
  roaring_bitmap.add(4294967294);  
  roaring_bitmap.add(4294967295);  
  roaring_bitmap.remove(3);  
  println!("roaring_bitmap: {:?} ", roaring_bitmap.values());  

  let mut roaring_bitmap64 = RoaringBitMap64::new();  
  roaring_bitmap64.add(1);  
  roaring_bitmap64.add(1);  
  roaring_bitmap64.add(2);  
  roaring_bitmap64.add(18446744073709551614);  
  roaring_bitmap64.add(18446744073709551615);  
  roaring_bitmap64.remove(3);  
  println!("roaring_bitmap64: {:?} ", roaring_bitmap64.values());  
