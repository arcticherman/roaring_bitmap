use std::collections::BTreeSet;

use crate::vec_set::VecSet;

pub struct RoaringSet{
    use_tree_set:    bool,
    vec:        VecSet,
    tree_set:   BTreeSet<u16>,
    mid_size:   usize,
    num:        usize,
}

impl RoaringSet{
    pub fn new() -> RoaringSet{
        return RoaringSet{
            use_tree_set:   true,
            vec:            VecSet::new(),
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
            self.vec.add(value);
            self.num = self.vec.len();
        }
        if self.num > self.mid_size && self.use_tree_set{
            //change to Vec
            println!("change to Vec");
            self.vec.clear();
            for elem in &self.tree_set{
                self.vec.add(*elem);
            }
            self.use_tree_set = false;
        }
    }

    pub fn remove(&mut self, value: u16){
        if self.use_tree_set{
            self.tree_set.remove(&value);
            self.num = self.tree_set.len();
        }else{
            self.vec.remove(value);
            self.num = self.vec.len();
        }
        if self.num < (self.mid_size>>1) && !self.use_tree_set{
            //change to BTreeSet
            println!("change to BTreeSet");
            self.tree_set.clear();
            let values = self.vec.values();
            for value in &values{
                self.tree_set.insert(*value);
            }
            self.vec.clear();
            self.use_tree_set = true;
        }
    }

    pub fn len(&self) -> usize{
        return  self.num;
    }

    pub fn values(& self) -> Vec<u16>{
        if self.use_tree_set{
            let mut tree_set_vec:Vec<u16> = Vec::new();
            for elem in &self.tree_set{
                tree_set_vec.push(*elem);
            }
            return  tree_set_vec;
        }
        return self.vec.values();
    }

}