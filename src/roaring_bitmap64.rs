use std::collections::BTreeMap;
use crate::roaring_set::RoaringSet;
pub struct RoaringBitMap64{
    tree_map:    BTreeMap<u64, RoaringSet>,
    num:        usize,
}
impl  RoaringBitMap64 {

    pub fn new() -> RoaringBitMap64{
        return RoaringBitMap64 {
            tree_map: BTreeMap::new(),
            num: 0,
        };
    }

    pub fn add(& mut self, value: u64){
        let slot_index = self.get_slot_index(value);
        let store_val = self.get_store_val(value);
        if ! self.tree_map.contains_key(&slot_index){
            self.tree_map.insert(slot_index, RoaringSet::new());
        }
        let roaring_set =  self.tree_map.get_mut(&slot_index).unwrap();
        let old_len = roaring_set.len();
        roaring_set.add(store_val);
        if roaring_set.len() > old_len{
            self.num = self.num + 1;
        }
    }

    pub fn remove(& mut self, value: u64){
        let slot_index = self.get_slot_index(value);
        let store_val = self.get_store_val(value);
        if self.tree_map.contains_key(&slot_index) {
            let mut roaring_set = self.tree_map.get_mut(&slot_index).unwrap();
            let old_len = roaring_set.len();
            roaring_set.remove(store_val);
            if roaring_set.len() < old_len{
                self.num = self.num - 1;
            }
        }
    }

    fn get_slot_index(&self, value: u64) -> u64{
        return value>>16;
    }

    fn get_store_val(&self, value: u64) -> u16{
        return (value & 0xFFFF) as u16;
    }

    pub fn values(&self) -> Vec<u64>{
        let mut vec:Vec<u64> = Vec::new();
        for (store_val, roaring_set) in self.tree_map.iter(){
            let set_values = roaring_set.values();
            let prefix_val =  (store_val << 16) as u64;
            for store_val in set_values{
                vec.push(prefix_val | (store_val as u64));
            }
        }
        return  vec;
    }

    pub fn len(&self) -> usize{
        return  self.num;
    }

}