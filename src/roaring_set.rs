use std::collections::BTreeSet;

pub struct RoaringSet{
    use_vec:    bool,
    vec:        Vec<bool>,
    tree_set:   BTreeSet<u16>,
    mid_size:   u16,
    num:        usize,
}

impl RoaringSet{
    pub fn new(){
        return RoaringSet{
            use_vec:    true,
            vec:        Vec::new(),
            tree_set:   BTreeSet::new(),
            mid_size:   1<<14,
        };
    }

    pub fn add(&mut self, value: u16){
        if self.use_vec{
            if value >= vec.len(){
                vec.resize(value + 1);
            }
            if !vec[value]{
                num = num + 1;
            }
            vec[value] = true;
        }else{
            tree_set.insert(value);
            num = tree_set.len()
        }
    }
}