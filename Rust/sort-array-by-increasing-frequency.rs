use std::cmp::Ordering;
use std::fmt;

struct Element {
    value: i32,
    counter: u32,
}

impl Ord for Element {
    fn cmp(&self, other: &Self) -> Ordering {
        if(self.counter == other.counter) {
            if self.value < other.value {
                return Ordering::Greater; 
            } else if self.value > other.value {
                return Ordering::Less; 
            }else {
                return Ordering::Equal; 
            }
        } else {
            return self.counter.cmp(&other.counter);
        }
    }
}

impl PartialOrd for Element {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        return Some(self.cmp(other));
    }
}

impl PartialEq for Element {
    fn eq(&self, other: &Self) -> bool {
        return (self.value, &self.counter) == (other.value, &other.counter);
    }
}

impl Eq for Element { }


impl Solution {
    pub fn frequency_sort(nums: Vec<i32>) -> Vec<i32> {
        let mut ret: Vec<i32> = Vec::new();
        let mut counters : Vec<Element> = Vec::new();
        
        for n in 0..201 {
            let mut el: Element = Element{
                value: n-100,
                counter: 0
            };
            counters.push(el);
        }

        for n in &nums {
            counters[*n as usize + 100].counter = counters[*n as usize + 100].counter+1;
        }
        
        counters.sort();
        
        for el in counters {
            for i in 0..el.counter {
                ret.push(el.value);
            }
        }

        return ret;
    }
}
