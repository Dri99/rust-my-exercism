pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let n = minefield.len();
    //let m = if n > 0 { minefield[0].len() } else {0};

    let mut mine_signed: Vec<String> = Vec::with_capacity(n);
    for (i, line) in minefield.iter().enumerate() {
        let mut new_line = String::with_capacity(minefield[i].len());

        let i_bounds = get_i_bounds(minefield,i);
        for j in 0..minefield[i].len() {


            let j_bounds = get_j_bounds(line,j);

            let mut count:u8 = 0;

            if line.as_bytes()[j] != b'*' {
                for k in i_bounds.0..=i_bounds.1 {
                    let row = minefield[k].as_bytes();
                    for l in j_bounds.0..=j_bounds.1 {
                        if row[l] == b'*' { count += 1;};
                    }
                }
            }


            let new_char :u8 = if count == 0 {line.as_bytes()[j]} else {b'0' + count};

            new_line.push(new_char as char);
        }
        mine_signed.push(new_line);

    }
    return mine_signed;
}

fn get_i_bounds(matrix: &[&str], i: usize ) -> (usize,usize) {
    let n = matrix.len();
    if n == 1 {
        return (0,0);
    }
    if i == 0 {
        return (0,1);
    } else if i == (n-1) { //this is the last row
        return (i-1,i);
    } else {
        return (i-1,i+1)
    }
    /*match i {
        0 => (0,1),
        n => (n-2,n),
        _ =>(i-1,i+1)
    }*/
}

fn get_j_bounds(row: &str, j: usize ) -> (usize,usize) {
    let m = row.len() ;
    if m == 1 {
        return (0,0);
    }
    if j == 0 {
        return (0,1);
    } else if j == (m-1) {
        return (j-1,j);
    } else {
        return (j-1,j+1)
    }
    /*match j {
        0 => (0,1),
        m if j == m => (m-2,m),
        _ =>(j-1,j+1)
    }*/
}

#[allow(dead_code)]
fn count_mine(near_field: &[&str]) -> u8 {
    let mut count:u8 = 0;
    for line in near_field {
        for c in line.chars() {
            if c == '*' { count += 1;}
        }
    }
    return count;
}
