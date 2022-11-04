use std::collections::BTreeSet;

pub struct RoaringSet{
    use_tree_set:    bool,
    vec:        Vec<bool>,
    tree_set:   BTreeSet<u16>,
    mid_size:   usize,
    num:        usize,
}

impl RoaringSet{
    pub fn new() -> RoaringSet{
        return RoaringSet{
            use_tree_set:   true,
            vec:            Vec::new(),
            tree_set:       BTreeSet::new(),
            mid_size:       1<<14,
            num:            0,
        };
    }

    pub fn add(&mut self, value: u16){
        if self.use_tree_set{
            self.tree_set.insert(value);
            self.num = self.tree_set.len();
        }else{
            let usize_val = value as usize;
            if usize_val >= self.vec.len(){
                self.vec.resize(usize_val + 1, false);
            }
            if !self.vec[usize_val]{
                self.num = self.num + 1;
            }
            self.vec[usize_val] = true;
        }
        if self.num > self.mid_size && self.use_tree_set{
            //change to Vec
            self.vec.resize(self.num, false);
            for i in 0..self.vec.len(){
                self.vec[i] = false;
            }
            for elem in &self.tree_set{
                self.vec[*elem as usize] = true;
            }
        }
    }

    pub fn remove(&mut self, value: u16){
        if self.use_tree_set{
            self.tree_set.remove(&value);
            self.num = self.tree_set.len();
        }else{
            let usize_val = value as usize;
            if usize_val < self.vec.len(){
                if self.vec[usize_val]{
                    self.num = self.num - 1;
                }
                self.vec[usize_val] = false;
            }
        }
        if self.num < (self.mid_size>>1) && !self.use_tree_set{
            //change to BTreeSet
            self.tree_set.clear();
            for i in 0..self.vec.len(){
                if self.vec[i]{
                    self.tree_set.insert(i as u16);
                }
            }
        }
    }

    pub fn len(&self) -> usize{
        return  self.num;
    }
}