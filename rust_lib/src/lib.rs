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
    //     age: person.age + 1, // 增加年龄
    // };

    // 创建一个新的CString，而不是接管 person.name 的所有权
    let name = unsafe { CStr::from_ptr(person.name).to_bytes().to_vec() };
    let name = CString::new(name).unwrap().into_raw();

    let new_person = Person {
        name,
        age: person.age + 1, // 增加年龄
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
    let error_message = "这是一个错误!!!!".to_string();
    let error = Error {
        error_code: -1,
        message: CString::new(error_message).unwrap().into_raw(),
    };
    Box::into_raw(Box::new(error))
}

