#[macro_use]
extern crate std;
use std::collections::HashMap;
use std::cell::RefCell;
use std::cell::UnsafeCell;
use std::ops::Deref;
use std::rc::Rc;


macro_rules! hash_map{
    ($($key:expr => $val:expr),*) => {
        {
            let mut map = HashMap::new();
            $(
                map.insert($key,$val);
            )*
            map
        }
    };
}


#[derive(Debug)]
struct SimpleStack<T>{
    stack:RefCell<Vec<T>>,
}

impl <T> SimpleStack<T>{
    fn new() -> SimpleStack<T>{
        SimpleStack{stack:RefCell::new(Vec::new()),}
    }

    fn push(&self,value:T){
        self.stack.borrow_mut().push(value);
    }

    fn pop(&self)->Option<T>{
        self.stack.borrow_mut().pop()
    }
}



struct MyRc<T>{
    ptr: *mut RcInner<T>,
}

struct RcInner<T>{
    count:usize,
    data:T,
}

impl<T> MyRc<T>{
    fn new(data: T) -> Self{
        let inner = RcInner{
            count:1,
            data,
        };
        let ptr = Box::into_raw(Box::new(inner));
        MyRc{ptr}
    }

    fn clone(&self)->Self{
        unsafe{
            (*self.ptr).count += 1;
        }
        MyRc{ ptr: self.ptr }
    }

    fn get_count(&self)->usize{
        unsafe{
            (*self.ptr).count
        }
    }
}

impl<T> Deref for MyRc<T> {
    type Target = T;
    fn deref(&self)->&Self::Target{
        unsafe{ &(*self.ptr).data }
    }
}

impl<T> Drop for MyRc<T>{
    fn drop(&mut self){
        unsafe{
            if(*self.ptr).count == 1{
                drop(Box::from_raw(self.ptr));
            }
            else{
                (*self.ptr).count-= 1;
            }
        }
    }
}


fn main() {

    //problem1:
    let map =hash_map!{
        "a"=>1,
        "b"=>2,
        "c"=>3
    };
    println!("this is problem1:");
    println!("{:?}",map);
    println!(" ");

    //problem2:
    println!("this is problem2:");
    let rc = MyRc::new(String::from("hello,world!"));
    println!("dereference:get the lenth of rc's data: hello,world! the lenth is {}",rc.len());
    println!("initially, the inner count is:{}",rc.get_count());
    let rc_copy = MyRc::clone(&rc);
    println!("after cloning, the inner count is:{}",rc.get_count());
    drop(rc_copy);
    println!("after droping,the inner count is:{}",rc.get_count());
    println!(" ");

    //problem3:
    println!("this is problem3:");
    let stack = SimpleStack::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    println!("after pushing 1,2,3. now pop them out");
    println!("{:?}  {:?}  {:?}",stack.pop(),stack.pop(),stack.pop());

    println!(" ");
    
}
