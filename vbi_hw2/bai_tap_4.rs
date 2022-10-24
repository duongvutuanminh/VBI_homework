// ====================================
// BÀI 4:
// Yêu cầu: Không cần quan tâm tới logic
// sửa vấn đề liên quan tới ownership

// =====================================


fn main() {
    let mut v = vec![1, 2, 3];

    go(&mut v);

    // still need v here, so I can't pass ownership to the "go' method above
    println!("{}", v.len())
}

//the ownership is returned
fn go(v: &mut Vec<i32>) -> &mut Vec<i32> {

    //clone v so we can push 4 to v later on
    for i in v.clone() {
        println!("{}", i);
    }
    v.push(4);
    return v;
}