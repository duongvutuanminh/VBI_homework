// ====================================
// BÀI 3:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================


fn main(){

}

pub fn iter_num(num: i32) -> bool {

    let num_str = num.to_string();
    let chars = num_str.chars(); 
    let len = chars.clone().count(); //chars does not moved this time, since we clone it to the heap 

    println!("Len = {:?}", len);

    for c in chars {             
        println!(">>> {:?}", c);
    }

    return true;
}