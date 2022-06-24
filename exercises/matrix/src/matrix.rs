
pub struct Row {
    data: Vec <i32>
}

impl Row {
    fn new() -> Self {
        Self {
            data:vec!(0,1)
        }
    }
}

impl Index<usize> for Row {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output { &self.data[index]}

    fn index_mut(&self, index: usize) -> &mut Self::Output { &mut self.data[index]}
}

/*impl Default for Row {
    fn default() -> Self {

    }
}*/

impl Clone for Row {
    fn clone(&self) -> Self {
        Self
    }
}
struct Matrix {
    data: Vec<Row>
}

impl Matrix {
    fn new() -> Self {
        Self {
            data : new(0,1)
        }
    }
}

impl Index<usize> for Matrix {
    type Output = i32;
    fn index(&self, index: usize) -> &Self::Output { &self.data[index]}

    fn index_mut(&self, index: usize) -> &mut Self::Output { &mut self.data[index]}
}