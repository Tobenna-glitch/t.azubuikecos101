struct rectangle {
    width;u32, height:u32
}


impl rectangle {
    fn are(&self)->u32 {
        self.width * self.height
    }
}

fn main() {
    let small = rectangle {
        width;10
        height:20
    };
    println!("width is {} \n height is {} \n area of rectangle is {}",
        smal.width,small.height,small.are());
}