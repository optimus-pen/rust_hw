


struct Buffer <T> {
    pub v: Vec<T>,
}

trait method <T>{
    fn sum(&self)-> Option<T>where T: std::ops::Add<Output = T> + Default + Copy;
    fn new() -> Self; 
}

impl<T> method<T> for Buffer<T>{
    fn new() -> Self {
        Buffer { v: Vec::new() }
    }
    fn sum(&self)-> Option<T>
    where
    T: std::ops::Add<Output = T> + Default + Copy,
    {
        if self.v.is_empty() {
            None
        } else {
            let mut total = T::default();
            for &number in &self.v{
                total = total + number;
            }
            Some(total)
        }
    }
}

fn compareString(x:&str,y:&str) -> bool{
    let mut iter1 = x.chars();
    let mut iter2 = y.chars();
    loop{
        match(iter1.next(),iter2.next()){
            (Some(c1),Some(c2))=>{
                  if c1>c2{
                    return true;
                  }
                  else if c1<c2{
                    return false;
                  }
            }
            (None,None)=>{
                return false;
            }
            (None,_)=>{
                return false;
            }
            (_,None)=>{
                return true;
            }
        }
    }
}

fn main() {

    //number1
    println!("This is hw number1:");
    let mut buffer1: Buffer<i32> = Buffer::new();
    let mut buffer2: Buffer<f32> = Buffer::new();

    println!("at fist the buffer1 and buffer2 is initialized.");
    if let Some(sum) = buffer1.sum() {
        println!("Sum: {}", sum);
    } else {
        println!("Buffer1 is empty");
    }
    if let Some(sum) = buffer2.sum() {
        println!("Sum: {}", sum);
    } else {
        println!("Buffer2 is empty");
    }

    buffer1.v.push(1);
    buffer1.v.push(2);
    println!("buffer1:after inserting 1 and 2");
    buffer2.v.push(1.9);
    buffer2.v.push(2.3);
    println!("buffer2:after inserting 1.9 and 2.3");

    if let Some(sum) = buffer1.sum() {
        println!("Sum: {}", sum);
    } else {
        println!("Buffer1 is empty");
    }
    if let Some(sum) = buffer2.sum() {
        println!("Sum: {}", sum);
    } else {
        println!("Buffer2 is empty");
    }
    println!(" ");


    //number2
    println!("This is hw number2:");
    let str1="hello";
    let str2="world";
    if compareString(str1, str2) {
        println!("'hello' is greater than 'world'");
    }
    else {
        println!("'hello' is not greater than 'world'");
    }
    println!(" ");

    println!("This is hw number3:");
    let data = vec!['a','b','c','d','e'];
    println!("the init data:{:?}",data);
    let result: Vec<char> = data.iter()
    .map(|&c| (c as u8 + 1) as char) // 将字符转换为 ASCII 值加 1 的字符
    .collect();
    println!("after operations, the data: {:?}", result);
}