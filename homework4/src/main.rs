// #[derive(Debug)]
// struct Vector<'a, T>{
//     arr: &'a [T],
//     length: usize,
//     mem: usize
// }

// impl<'a, T> Vector<'a, T>{
//     pub fn new(/*mem: Option<i32>*/) -> Self {
//         Vector{
//             arr: &[],
//             length: 0,
//             mem: 1
//         }
//     }
//     pub fn push(self, x: T){
//         if self.length == self.mem {
//             let temp = [T; (self.mem)*2];
//         }
//         self.arr[self.length] = x;
//         self.length+=1;
//     }
// }

struct Vector<T> {
    ptr: *mut T, // core::ptr
    len: usize,
    mem: usize,
}

impl<T> Vector<T> {
    fn new() -> Vector<T> {
        Vector {
            ptr: std::ptr::null_mut(), //Creates a null mutable raw pointer.
            len: 0,
            mem: 0,
        }
    }

    fn push(&mut self, value: T) {
        if self.len == self.mem {
            let new_mem: usize = {
                if self.mem == 0 { 1 }
                else {self.mem * 2}
            };
            // if self.mem > 0 { // удвоить размер массива, если нет места
            //     new_mem: usize = 1;
            // } else {
            //     let new_mem = self.mem * 2;
            // }

            let size = new_mem * std::mem::size_of::<T>();
            let align = std::mem::align_of::<T>(); // адрес
            let new_ptr = unsafe{ // новая ячейка
                std::alloc::alloc(std::alloc::Layout::from_size_align_unchecked(size, align)) as *mut T
            };

            for i in 0..self.len { // запись в новую ячейку
                unsafe {
                    std::ptr::write(
                        new_ptr.offset(i as isize),
                        std::ptr::read(self.ptr.offset(i as isize))
                    );
                }
            }

            // удаление старого массива
            let prev_size = self.mem * std::mem::size_of::<T>();
            let prev_align = std::mem::align_of::<T>();
            unsafe {
                let layout = std::alloc::Layout::from_size_align_unchecked(prev_size, prev_align);
                std::alloc::dealloc(self.ptr as *mut u8, layout);
            }

            self.ptr = new_ptr;
            self.mem = new_mem;
        }

        // добавление элемента
        unsafe {
            std::ptr::write(self.ptr.offset(self.len as isize), value);
        }
        self.len += 1;
    }

    fn get(&self, index: usize) -> Option<&T> {
        if index < self.len {
            unsafe { 
                Some(&*self.ptr.offset(index as isize))
            }
        } else {
            None
        }
    }
    

    fn pop_back(&mut self) -> Option<&T> {
        if self.len > 0 {
            self.len -= 1;
            
            // todo: real delete
            // let layout = std::alloc::Layout::
            // std::alloc::dealloc(ptr, layout);

            unsafe { 
                Some(&*self.ptr.offset(self.len as isize)) 
            }
        } else {
            None
        }
    }

}


fn main(){
    println!("Hello, world!");
    let mut  v: Vector<i32> = Vector::new();
    for i in 1..11{
        v.push(i);
    }
    for i in 0..10{
        if let Some(value) = v.get(i){
            println!("{}",value)
        } else{
            panic!()
        }
    }
    println!("deleting");
    for _ in 0..10{
        if let Some(value) = v.pop_back(){
            println!("{}",value)
        } else{
            panic!()
        }
    }
    // println!("{}",arr.get(1));
    println!("end");
}