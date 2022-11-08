use std::vec;

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
        let store_value = self.get_store_value(value);
        if slot_index >= self.data.len(){
            self.data.resize(slot_index + 1, 0);
        }
        if (self.data[slot_index] & store_value) == 0{
            self.num = self.num + 1;
            self.data[slot_index] = self.data[slot_index] | store_value;
        }
    }

    pub fn remove(&mut self, value: u16){
        let slot_index = self.get_slot_index(value);
        let store_value = self.get_store_value(value);
        if slot_index < self.data.len(){
            if (self.data[slot_index] & store_value) > 0{
                self.num = self.num - 1;
                self.data[slot_index] = self.data[slot_index] & (0xFFFF ^ store_value);
            }
        }
    }

    fn get_slot_index(&self, value: u16) -> usize{
        return (value>>4) as usize;
    }

    fn get_store_value(&self, value: u16) -> u16{
        return 1<<(value & 0xF);
    }

    pub fn len(&self) -> usize{
        return  self.num;
    }

    pub fn values(&self) -> Vec<u16>{
        let mut vec:Vec<u16> = Vec::new();
        for i in 0..self.data.len(){
            let mut val = self.data[i];
            let k = 1<<4;
            for j in 0..k{
                if val == 0{
                    break;
                }
                if (val & (1<<j)) > 0{
                    let store_value:u16 = ((i<<4) | j) as u16;
                    vec.push(store_value);
                    val = val & (0xFFFF ^ (1<<j));
                }
            }
        }
        return  vec;
    }
    
    pub fn clear(&mut self){
        self.data.clear();
        self.num = 0;
    }
}