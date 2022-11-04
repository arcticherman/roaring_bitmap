use std::vec;
pub struct RoaringBitMap{
    pub bit_map:    Vec<Vec<bool>>,
    val_mask:      u32,
}
impl  RoaringBitMap {

    pub fn new() -> RoaringBitMap{
        return RoaringBitMap {
            bit_map: Vec::with_capacity(16),
            val_mask: (1<<16)-1,
        };
    }

    pub fn add(& mut self, _val: u32){
        let slot_index = self.get_slot_index(_val);
        let store_val = self.get_store_val(_val);
        println!("slot_index:{} store_val:{}", slot_index, store_val);
        if slot_index >= self.bit_map.len() {
            self.bit_map.resize(slot_index + 1, Vec::with_capacity(16));
            let usize_store_val = store_val as usize;
            if usize_store_val >= self.bit_map[slot_index].len(){
                self.bit_map[slot_index].resize(usize_store_val + 1, false);
            }
            self.bit_map[slot_index][usize_store_val] = true
        }
    }

    fn get_slot_index(&self, _val: u32) -> usize{
        return (_val>>16) as usize;
    }

    fn get_store_val(&self, _val: u32) -> u16{
        return (_val & self.val_mask) as u16;
    }
}