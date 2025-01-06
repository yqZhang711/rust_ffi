use wasm_bindgen::prelude::*;

// 定义一个结构体
#[wasm_bindgen]
pub struct Person {
    name: String,
    age: u32,
}

#[wasm_bindgen]
impl Person {
    // 构造函数
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: u32) -> Person {
        Person { name, age }
    }

    // 返回结构体的字段
    #[wasm_bindgen(getter)]
    pub fn name(&self) -> String {
        self.name.clone()
    }

    #[wasm_bindgen(getter)]
    pub fn age(&self) -> u32 {
        self.age
    }

    // 修改结构体的字段
    #[wasm_bindgen(setter)]
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    #[wasm_bindgen(setter)]
    pub fn set_age(&mut self, age: u32) {
        self.age = age;
    }

    // 返回一个修改后的结构体
    pub fn grow_older(&self) -> Person {
        Person {
            name: self.name.clone(),
            age: self.age + 1,
        }
    }
}