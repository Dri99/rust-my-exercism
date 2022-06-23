

/// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    //let mut vec = Vec::with_capacity(16);
    //unimplemented!("Is the Luhn checksum for {} valid?", code);
    let clean_code = code.replace(" ","");
    if clean_code.len() < 2 {
        return false;
    }

    let mut even_pos = false;
    let mut sum : u32 = 0;
    for c in clean_code.chars().rev() {
        if c.is_digit(10) {
            let mut raw_num : u32 = c.to_digit(10).unwrap();
                /*match c {
                c if c.is_digit(10) => {

                    if odd_pos {
                        odd_pos != odd_pos;
                        c.to_digit(10).unwrap() * 2 % 9;
                    } else {
                        odd_pos != odd_pos;
                        c.to_digit(10).unwrap() ;
                    }

                },
                ' ' => 0,
                _ => return false
            };*/
            if even_pos {
                raw_num = raw_num * 2;
                if raw_num > 9 {
                    raw_num = raw_num -9;
                }
            }
            sum += raw_num;
            even_pos = !even_pos;
        } else {
            return false;
        }

    }
    (sum % 10) == 0
}
