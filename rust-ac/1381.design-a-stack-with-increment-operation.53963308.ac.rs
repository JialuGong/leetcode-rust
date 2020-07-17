struct CustomStack {
    stack_vec:Vec<i32>,
    max_size:i32,

}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl CustomStack {

    fn new(maxSize: i32) -> Self {
        return CustomStack{stack_vec:Vec::new(),max_size:maxSize};
        
    }
    
    fn push(&mut self, x: i32) {
        if self.stack_vec.len()<self.max_size as usize{
           self.stack_vec.push(x);
        }

    }
    
    fn pop(&mut self) -> i32 {
        if !self.stack_vec.is_empty(){
            match self.stack_vec.pop() {
                Some(num) => {return num;},
                None => {return -1;},
            }
        }else{
            return -1;
        }

    }
    
    fn increment(&mut self, k: i32, val: i32) {
        let mut i=0;
        for num in &mut self.stack_vec{
            if i<k{
                *num+=val;
            }
            i+=1;
        }
    }
}

