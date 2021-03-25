use std::cmp::Ordering;
use std::str;
use wasm_bindgen::prelude::*;

fn main() {
    let data = "3,228,184,173,3,228,184,173,1,97,1,48";
    let a = uncode(&data);
    println!("{}", a);
}

#[wasm_bindgen]
extern "C" {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

fn swap(vec: Vec<u8>, i: usize, j: usize) -> Vec<u8> {
    let mut v = vec.clone();
    let len = v.len();
    if len < i + 1 || len < j + 1 {
        panic!("swap: i:{},j:{} is not fit!", i, j);
    }
    if let Some(v_i) = v.get_mut(i) {
        *v_i = vec[j]; 
    }
    if let Some(v_j) = v.get_mut(j) {
        *v_j = vec[i];
    }
    v
}

fn rebuild(str: &str) -> String{
    let split = str.split(",");
    let mut result = String::new();
    let mut counter = 0;
    let mut index_two = 0;
    let mut counter_two_active = false;
    let mut index_three = 0;
    let mut counter_three_active = false;
    let mut index_four = 0;
    let mut counter_four_active = false;
    let mut index = 0;
    let mut arr_inner: Vec<u8> = Vec::new();

    for key in split {
        let number = key.parse::<i32>().unwrap();
        match number.cmp(&5) {
            Ordering::Less => if number == 2 { index_two += 1; counter = number; } else if number == 1 { counter = number; } else if number == 3 { index_three += 1; counter = number; } else if number == 4 { index_four += 1; counter = number; },
            Ordering::Greater => arr_inner.push(number as u8),
            Ordering::Equal => println!(""),
        }

        if index_two % 4 == 0 {
            counter_two_active = true;
        } else {
            counter_two_active = false;
        }

        if index_three % 7 == 0 {
            counter_three_active = true;
        } else {
            counter_three_active = false;
        }

        if index_four % 9 == 0 {
            counter_four_active = true;
        } else {
            counter_four_active = false;
        }

        if counter == index {
            if counter == 2 {
                if counter_two_active == true {
                    arr_inner = swap(arr_inner, 0, 1);
                }
            } else if counter == 3 {
                if counter_three_active == true {
                    arr_inner = swap(arr_inner, 0, 2);
                }
            } else if counter == 4 {
                if counter_four_active == true {
                    arr_inner = swap(arr_inner, 0, 3);
                    arr_inner = swap(arr_inner, 1, 2);
                }
            }

            arr_inner.insert(0, counter as u8);

            for &inner_value in &arr_inner {
                let x: u32 = inner_value.into();
                let s: String = x.to_string();
                result.push_str(&s);
                result.push_str(",");
            }
        }

        if counter == index {
            counter = 0;
            index = 0;
            arr_inner.clear();
        } else {
            index += 1;
        }
    }

    result.remove(result.len() - 1);

    println!("{}", result);

    return result;
}

#[wasm_bindgen]
pub fn uncode_long(str2: &str) -> String {
    let mut result = String::new();
    let mut counter = 0;
    let split = str2.split(",");
    let mut arr_inner: Vec<u8> = Vec::new();

    for key in split {
        let number = key.parse::<i32>().unwrap();
        match number.cmp(&4) {
            Ordering::Less => println!(""),
            Ordering::Greater => arr_inner.push(number as u8),
            Ordering::Equal => println!(""),
        }

        if counter == 3 {
            let string_one = str::from_utf8(&arr_inner).unwrap();
            println!("{}", string_one);
            result.push_str(string_one);
        }

        if counter == 3 {
            counter = 0;
            arr_inner.clear();
        } else {
            counter += 1;
        }
    }
    return result;
}

#[wasm_bindgen]
pub fn uncode(str2: &str) -> String {
    let builded_string = rebuild(str2);
    let mut result = String::new();
    let mut counter = 0;
    let mut index = 0;
    let split = builded_string.split(",");
    let mut arr_inner: Vec<u8> = Vec::new();

    for key in split {
        let number = key.parse::<i32>().unwrap();
        match number.cmp(&4) {
            Ordering::Less => counter = number,
            Ordering::Greater => arr_inner.push(number as u8),
            Ordering::Equal => println!(""),
        }

        if counter == index {
            let string_one = str::from_utf8(&arr_inner).unwrap();
            result.push_str(string_one);
        }

        if counter == index {
            counter = 0;
            index = 0;
            arr_inner.clear();
        } else {
            index += 1;
        }
    }
    return result;
}