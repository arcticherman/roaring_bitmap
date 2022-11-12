use std::vec;
use crate::roaring_set::RoaringSet;
pub struct RoaringBitMap{
    vec_set:    Vec<RoaringSet>,
    num:        usize,
}
impl  RoaringBitMap {

    pub fn new() -> RoaringBitMap{
        return RoaringBitMap {
            vec_set: Vec::with_capacity(16),
            num: 0,
        };
    }

    pub fn add(& mut self, value: u32){
        let slot_index = self.get_slot_index(value);
        let store_val = self.get_store_val(value);
        if slot_index >= self.vec_set.len() {
            self.vec_set.resize(slot_index + 1, RoaringSet::new());
        }
        let old_len = self.vec_set[slot_index].len();
        self.vec_set[slot_index].add(store_val);
        if self.vec_set[slot_index].len() > old_len{
            self.num = self.num + 1;
        }
    }

    pub fn remove(& mut self, value: u32){
        let slot_index = self.get_slot_index(value);
        let store_val = self.get_store_val(value);
        if slot_index < self.vec_set.len() {
            let old_len = self.vec_set[slot_index].len();
            self.vec_set[slot_index].remove(store_val);
            if self.vec_set[slot_index].len() < old_len{
                self.num = self.num - 1;
            }
        }
    }

    fn get_slot_index(&self, value: u32) -> usize{
        return (value>>16) as usize;
    }

    fn get_store_val(&self, value: u32) -> u16{
        return (value & 0xFFFF) as u16;
    }

    pub fn values(&self) -> Vec<u32>{
        let mut vec:Vec<u32> = Vec::new();
        for i in 0..self.vec_set.len(){
            let set_values = self.vec_set[i].values();
            let prefix_val =  (i << 16) as u32;
            for store_val in set_values{
                vec.push(prefix_val | (store_val as u32));
            }
        }
        return  vec;
    }

    pub fn len(&self) -> usize{
        return  self.num;
    }

}