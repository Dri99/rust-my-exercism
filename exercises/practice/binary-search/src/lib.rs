
pub fn find(array: &[i32], key: i32) -> Option<usize> {
    rec_find(array,key)

}
#[allow(dead_code)]
pub fn iter_find(array: &[i32], key: i32) -> Option<usize> {

    let mut slice = array;
    let mut index:usize = 0;

    while slice.len() > 1 {
        let i_split = slice.len()/2;
        let (left,right) = slice.split_at(i_split);
        //if key > *left.last().unwrap_or( &i32::MIN)
        if key > left[left.len()-1] {
            slice = right;
            index = index + i_split;
        } else {
            slice = left;
        }
    }
    if !slice.is_empty() && slice[0] == key {
        assert_eq!(array[index],key,"The algorithm is broken");
        return Some(index);
    }
    return None;
}

/*impl Add for Option<usize>{
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        if self.is_none() || rhs.is_none() {
            None
        }else {
            self.unwrap() + rhs.unwrap()
        }
    }
}*/
#[allow(dead_code)]
fn rec_find(array: &[i32], key: i32) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }

    if array.len() == 1 {
         return if array[0] == key {
            Some(0)
        } else {
            None
        }
    }

    let i_split = array.len()/2;
    let (left,right) = array.split_at(i_split);

    let (found,i_index) = if key > left[left.len()-1] {
        (rec_find(right, key), i_split)
    } else {
        (rec_find(left, key), 0)
    };

    if found.is_some() {
        return Some(found.unwrap() + i_index )
    }
    return None;
}
