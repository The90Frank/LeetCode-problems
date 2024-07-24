use std::cmp::Ordering;
use std::fmt;

struct Element {
    value: i32,
    idx: usize,
    mappedvalue: i32,
}

impl Element {
    fn calc_mapping(value: i32, mapping: &Vec<i32>) -> i32 {
        if (value == 0) {
            return mapping[0];
        }
        let mut ret = 0;
        let mut currentexp = 1;
        while (value/currentexp) > 0 {
            ret = ret + (mapping[((value/currentexp)%10) as usize]*currentexp);
            currentexp = currentexp * 10;
        }
        return ret;
    }

    pub fn new(value: i32, idx: usize, mapping: &Vec<i32>) -> Self {
        Element {
            value: value,
            idx: idx,
            mappedvalue: Self::calc_mapping(value, mapping)
        }
    }
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        if(self.mappedvalue == other.mappedvalue) {
            return self.idx.cmp(&other.idx);
        }
        return self.mappedvalue.cmp(&other.mappedvalue);
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        return (self.value, &self.mappedvalue) == (other.value, &other.mappedvalue);
    }
}

impl Eq for Element { }


impl Solution {
    pub fn sort_jumbled(mapping: Vec<i32>, nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut mapped : Vec<Element> = Vec::new();
        
        for i in 0..nums.len() {
            let mut el: Element = Element::new(nums[i], i, &mapping);
            mapped.push(el);
        }

        mapped.sort();
        
        for el in mapped {
            ret.push(el.value);
        }

        return ret;
    }
}
