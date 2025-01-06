# Go如何调用Rust
- rust起个服务，对外提供接口，golang通过接口调用
   1. 优点: 不需要直接处理内存管理和类型转换。
   2. 缺点: 需要额外的系统维护，性能低
   
   想到的最简单的方法，但是这肯定不是我们想要的

- 通过共享内存
   共享内存或消息队列（如:Redis）进行通信
   优缺点同上面一样，额外引入其他服务，系统稳定性差，肯定也不是我们想要的

- RUST FFI. 将RUST编译成动态库，golang通过调用动态库
  
  [RUST中文社区](https://rustcc.cn/search?q=Rust%20FFI%20%E7%BC%96%E7%A8%8B)，其中讲的大部分都是RUST和C之间的调用，其中一片也讲解了Python如何调用Rust，是通过python的库cffi, 还是使用Rust提供C ABI。

  此时想到了cgo，之前浅浅的了解了下cgo，cgo可以很方便的调用C, 那么是否可以用RUST提供C ABI，然后使用cgo去调用RUST。

  其实网上查到的很多Go调用Rust，go中都导入import "C"，感觉应该是这个方向。

# Rust FFI
- 写rust ffi时extern是什么
   - extern "C" 是一种用于定义与 C 语言进行链接的接口的机制。这主要用于与 C 库进行交互，确保 Rust 使用 C 的调用约定。
   - 除了 extern "C"，Rust 还支持其他几种不同的调用约定（calling conventions）。它们定义了函数如何传递参数以及如何从函数中返回值。 
  - extern "C"：与 C 语言兼容的调用约定。
  - extern "stdcall"：Windows 平台的标准调用约定。
  - extern "fastcall"：Windows 平台的另一种优化过的调用约定。
  - extern "sysv64" 和 extern "sysv32"：用于与 Unix-like 系统进行交互的调用约定。
  - extern "unwind"：与异常处理和栈展开相关的调用约定。

    没有extern "golang"！！！

- #[no_mangle]：防止Rust编译器对函数名进行名称修饰，以保证在生成的C共享库中，Rust函数可以保持原名。
- Rust Box
  ```
  fn main() {
    let b = Box::new(5);
    
    // 使用 `Box::into_raw` 获取裸指针
    let raw_ptr: *mut i32 = Box::into_raw(b);

    // 注意：此时 `b` 不再有效，因为它的所有权已经转移给裸指针
    // 我们需要手动处理这个裸指针，不能继续使用 `b`。

    unsafe {
        // 通过裸指针解引用
        println!("raw_ptr points to: {}", *raw_ptr);
    }

    // 最后，手动释放内存
    unsafe {
        // 使用 `Box::from_raw` 来将裸指针转换回 `Box`，从而自动释放内存
        let _boxed_again = Box::from_raw(raw_ptr);
    }
  }
  ```
  1. Box::new(5) 创建了一个 Box，它将整数 5 存储在堆上。
   
  2. Box::into_raw(b) 将 Box 的所有权转移给裸指针 raw_ptr，此时 b 不再有效，raw_ptr 指向堆上的数据。
   
  3. 使用 unsafe 块来解引用裸指针，访问堆上的数据。裸指针绕过了 Rust 的借用检查，所以需要谨慎使用。
   
  4. 使用 Box::from_raw(raw_ptr) 来将裸指针重新转换成 Box，这样会自动释放原来的堆内存。
   
  5. Rust 的所有权系统会有效地管理堆内存，而 Box::into_raw 是在你需要将内存交给其他系统（如 C 库）或者需要更精细的内存控制时使用的工具。它让你能在 Rust 中通过裸指针进行更低级别的操作。
  6. ***注意!!! 使用裸指针时，你需要手动管理内存，避免内存泄漏或悬挂指针等问题。***

 - #[repr(C)]
   
    Rust 中，#[repr(C)] 是一个属性（attribute），它用于指定结构体或枚举的内存布局遵循 C 语言的标准布局规则。这个属性的主要作用是确保 Rust 类型的内存布局与 C 语言中的类型布局兼容，以便在 Rust 和 C 之间进行数据交换或调用。
    Rust 在对齐和填充方面有自己的优化策略，这可能会导致结构体在内存中的布局与 C 语言中相应类型的布局不一致。#[repr(C)] 让 Rust 类型的内存布局与 C 类型的内存布局一致，这对于 FFI（Foreign Function Interface，外部函数接口）非常重要，特别是当需要与 C 语言代码共享结构体或进行内存映射时。
    ```
    #[repr(C)]
    struct MyStruct {
        a: i32,
        b: f32,
    }
    # 结构体会使用 C 语言的布局规则进行排列。这意味着 a 和 b 在内存中的排列顺序将严格按照 C 语言的标准进行，即 a 将紧接着 b，并且按照 C 编译器的对齐要求。
    ```
    Rust 提供了多种 repr 属性，每种都用于控制内存布局的方式：
     1. #[repr(C)]：指定结构体或枚举遵循 C 语言的布局。
   
     2. #[repr(packed)]：控制结构体成员不进行填充，以节省内存空间，但这可能会影响性能，因为对齐要求可能不满足。
   
     3. #[repr(transparent)]：用于标记单一字段的结构体，这样它的大小和布局就像该字段一样，而不添加额外的结构体头部。

    这中间的：布局，排列顺序是什么，各语言有什么不同，可能就得进一步深究了。目前没搞懂
  
## 创建Rust动态库流程
- 新建rust项目
  ```
  cargo new --lib rsut_lib
  ```
  修改配置文件
  ```
  [package]
  name = "rsut_lib"
  version = "0.1.0"
  edition = "2021"

  [dependencies]

  [lib]
  name = "rsut_lib"
  crate-type = ["cdylib"]  # 表示编译为 C 兼容的动态库
  # crate-type = ["staticlib"]  # 创建 C 兼容的静态库
  ```
- 写Rust代码
  在src/lib.rs中写Rust代码
  ```
    // src/lib.rs
    use std::ffi::{CStr, CString};
    use std::os::raw::c_char;

    #[repr(C)]
    pub struct Person {
        pub name: *mut c_char,
        pub age: i32,
    }

    // 问题：直接复制了 name 指针，这会导致多个 Person 实例共享同一个 name 指针，从而在释放时可能导致双重释放（double free）问题。
    // impl Clone for Person {
    //     fn clone(&self) -> Self {
    //         Person {
    //             name: unsafe { CString::from_raw(self.name).into_raw() },
    //             age: self.age,
    //         }
    //     }
    // }
    impl Clone for Person {
        fn clone(&self) -> Self {
            let name = unsafe { CStr::from_ptr(self.name).to_bytes().to_vec() };
            let name = CString::new(name).unwrap().into_raw();
            Person {
                name,
                age: self.age,
            }
        }
    }

    // 错误类型，定义为返回错误信息的方式
    #[repr(C)]
    pub struct Error {
        pub error_code: i32,
        pub message: *mut c_char,
    }

    // 使用`extern "C"`来暴露C ABI接口
    #[no_mangle]
    pub extern "C" fn create_person(name: *const c_char, age: i32) -> *mut Person {
        // 将C字符串转换为Rust字符串
        let c_str = unsafe { CStr::from_ptr(name) };
        let name_str = match c_str.to_str() {
            Ok(str) => str,
            Err(_) => return std::ptr::null_mut(), // 如果C字符串无效，返回null
        };

        // 创建并返回一个新的Person对象
        let person = Person {
            name: CString::new(name_str).unwrap().into_raw(),
            age,
        };

        Box::into_raw(Box::new(person))
    }

    #[no_mangle]
    pub extern "C" fn get_person_details(person: *mut Person) -> *mut Person {
        if person.is_null() {
            return std::ptr::null_mut();
        }

        let person = unsafe { &*person }; // 使用不可变引用
        if person.age < 18 {
            return std::ptr::null_mut(); // 如果年龄小于18，返回null
        }

        // 创建一个新的Person对象, 这里接管了 person.name 的所有权
        // let new_person = Person {
        //     name: unsafe { CString::from_raw(person.name).into_raw() },
        //     age: person.age + 1, 
        // };

        // 创建一个新的CString，而不是接管 person.name 的所有权
        let name = unsafe { CStr::from_ptr(person.name).to_bytes().to_vec() };
        let name = CString::new(name).unwrap().into_raw();

        let new_person = Person {
            name,
            age: person.age + 1, 
        };

        Box::into_raw(Box::new(new_person))
    }

    #[no_mangle]
    pub extern "C" fn free_person(person: *mut Person) {
        if person.is_null() {
            return;
        }
        unsafe {
            let _ = CString::from_raw((*person).name); // 释放name
            let _ = Box::from_raw(person); // 释放Person
        }
    }

    #[no_mangle]
    pub extern "C" fn get_error_message() -> *mut Error {
        let error_message = "这是一个错误".to_string();
        let error = Error {
            error_code: -1,
            message: CString::new(error_message).unwrap().into_raw(),
        };
        Box::into_raw(Box::new(error))
    }
  ```

- 将rust编译成动态库
  ```
  # 编译后的共享库将位于target/release目录下，名称根据操作系统不同可能是librust_cgo_example.so（Linux），librust_cgo_example.dylib（macOS），或rust_cgo_example.dll（Windows）。
  cargo build --release

  # 如果是静态库:编译后，静态库通常会生成 .a 文件（在 Linux 或 macOS 上）或 .lib 文件（在 Windows 上）。例如：libyour_project_name.a 或 your_project_name.lib。
  ```

- 编写golang代码
  ```
    package main

    /*
    #cgo LDFLAGS: -L. -lrust_lib
    #include <stdint.h>
    #include <stdlib.h>

    struct Person {
        char* name;
        int age;
    };

    struct Error {
        int error_code;
        char* message;
    };

    extern struct Person* create_person(const char* name, int age);
    extern struct Person* get_person_details(struct Person* person);
    extern void free_person(struct Person* person);
    extern struct Error* get_error_message();
    */
    import "C"
    import (
        "fmt"
        "unsafe"
    )

    func main() {
        // 创建一个Person
        name := C.CString("Alice")
        defer C.free(unsafe.Pointer(name))

        person := C.create_person(name, 30)
        defer C.free_person(person)

        if person == nil {
            fmt.Println("Failed to create person")
            return
        }

        // 获取并修改Person
        updatedPerson := C.get_person_details(person)
        if updatedPerson == nil {
            fmt.Println("Error: Invalid person or age less than 18")
        } else {
            fmt.Printf("Updated person: %s, Age: %d\n", C.GoString(updatedPerson.name), updatedPerson.age)
            defer C.free_person(updatedPerson)
        }

        // 错误处理示例
        errorMessage := C.get_error_message()
        defer func() {
            if errorMessage != nil {
                C.free(unsafe.Pointer(errorMessage.message))
                C.free(unsafe.Pointer(errorMessage))
            }
        }()
        if errorMessage != nil {
            fmt.Printf("Error code: %d, Message: %s\n", errorMessage.error_code, C.GoString(errorMessage.message))
        }
    }

  ```
  
## 创建Rust动态库流程

修改配置文件
  ```
  [package]
  name = "rsut_lib"
  version = "0.1.0"
  edition = "2021"

  [dependencies]

  [lib]
  name = "rsut_lib"
  crate-type = ["staticlib"]  # 创建 C 兼容的静态库
```
差别就是动态库：crate-type = ["cdylib"]； 静态库：crate-type = ["staticlib"]，其他的都不变

## 动态库和静态库的区别

- 文件名后缀不一样
    1. 动态库：Linux: .so， macOS: .dylib， Windows: .dll  
    2. 静态库：inux/macOS: .a， Windows: .lib
   
- 加载方式
    1. 动态库：运行时被加载到内存中, 所以会增加启动golang程序的时间
    2. 静态库：静态库在编译时被直接嵌入到可执行文件中
   
- 文件大小: 通过加载方式可以看出，编译时将文件编译进去，所以静态 库便后后的go来那个程序更大
  
- 内存占用: 同上，静态占用内存高
  
- 更新：如果Rust改变了
   1. 动态库: 直接替换.so文件。不需要重新编译golang, 但是需要重启golang
   2. 静态库: 需要替换静态库，并重新编译golang, 并重启golang
   
- 性能：将上述代码，动态库和静态库测试调用100次, 静态库耗时短，所以静态库性能更好
   1. 动态库耗时：1.646162ms，每次调用都需要通过动态链接器解析函数地址。
   2. 静态库耗时：765.764µs，调用是直接的，没有额外的跳转开销。
   
## rust ffi实现的难点
 - 对C语言需要一定了解
  
 - rust, go, c之间的类型转换比较费劲
  
 - 内存释放问题，在代码中我有备注，理解不深刻非常容易报错，并且不易找到错误

# 是否还有其他方式实现Go调用Rust
  在"Rust中文社区中" 还提到过一种方式：Rust + WebAssembly(Wasm)，不过没找到相关实现。
  
  

