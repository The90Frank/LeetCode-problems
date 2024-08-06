use std::cmp::Ordering;
use std::fmt;

struct Element {
    value: char,
    counter: usize,
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
    pub fn minimum_pushes(word: String) -> i32 {
        let a_byte : usize = 97;
        let button_size : usize = 8;
        let my_ascii_lower = "abcdefghijklmnopqrstuvwxyz";
        let mut ret_counter = 0;
        let mut counters : Vec<Element> = Vec::new();
        
        for x in my_ascii_lower.chars() {
            let mut el: Element = Element{
                value: x,
                counter: 0
            };
            counters.push(el);
        }

        for x in word.as_bytes(){
            counters[(*x as usize - a_byte) as usize].counter = counters[(*x as usize - a_byte) as usize].counter+1;
        }

        counters.retain(|x| x.counter!=0);
        counters.sort();

        let mut click : usize = 0;
        let mut click_counter : usize = 0;
        
        for idx in (0..counters.len()).rev() {
            if click_counter % button_size == 0 {
                click += 1;
            }

            ret_counter += (counters[idx].counter * click);
            click_counter += 1;
        }

        return ret_counter as i32;
    }
}
