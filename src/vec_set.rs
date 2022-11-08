pub struct VecSet{
    data:   Vec<u16>,
    num:    usize,
}

impl VecSet{
    pub fn new() -> VecSet{
        return VecSet{
            data: Vec::new(),
            num: 0,
        };
    }

    pub fn add(&mut self, value: u16){
        let slot_index = self.get_slot_index(value);
        let slot_value = self.get_store_value(value);
        if slot_index >= self.data.len(){
            self.data.resize(slot_index + 1, 0);
        }
        if (self.data[slot_index] & slot_value) == 0{
            self.num = self.num + 1;
        }
        self.data[usize_val] = true;
    }

    fn get_slot_index(&self, value: u16) -> usize{
        return (value>>4) as usize;
    }

    fn get_store_value(&self, value: u16) -> u16{
        return value & 0xFF;
    }
}