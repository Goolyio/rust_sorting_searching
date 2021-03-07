use std::cmp::Ordering;

pub trait Sortable {
    fn sort(list: &mut Vec<i32>) -> Result<(), String>;
}

pub struct BubbleSort;

impl Sortable for BubbleSort {
    fn sort(list: &mut Vec<i32>) -> Result<(), String> {
        let list_length = list.len();
        if list_length <= 1 { return Ok(()); }

        let mut sorted = false;
        while !sorted {
            sorted = true;
            for i in 1..list_length {
                if i != list_length {
                    if list[i - 1] > list[i] {
                        list.swap(i - 1, i);
                        sorted = false;
                    }
                }
            }
        }

        return Ok(());
    }
}

pub struct QuickSort;

impl Sortable for QuickSort {
    fn sort(list: &mut Vec<i32>) -> Result<(), String> {
        let list_length: usize = list.len();
        if list_length <= 1 { return Ok(()); }
        
        QuickSort::do_sort(self, &mut list);
        return Ok(());
    }
}

impl QuickSort {
    pub fn do_sort(&self, list: &mut [i32]) -> Ordering {
        let list_length: usize = list.len();
        
        let pivot = list_length / 2;
        let mut left: usize = 1;
        let mut right: usize = list.len() - 1;
        
        loop {
            while left < list_length && list[left].cmp(&list[pivot]) != Ordering::Greater {
                left += 1;
            }
            
            while right > 0 && list[right].cmp(&list[pivot]) != Ordering::Less {
                right -= 1;
            }
            
            if left >= right {
                break;
            }
            
            list.swap(left, right);
            left += 1;
            right -= 1;
        }
        list.swap(pivot, right);
        self.do_sort(&mut list[0..min(left - 1, right)]);
        self.do_sort(&mut list[max(left, right + 1)..])
    }
}