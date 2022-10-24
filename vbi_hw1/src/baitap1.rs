pub fn bai_tap1() {
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,10];
    let l1 = org_arr.len();
    let l2 = sub_arr.len();
    let mut res: bool = false;

    let mut i = 0;
    let mut j = 0;
    while i<l1 && j<l2{
        if org_arr[i] == sub_arr[j]{
            i += 1;
            j += 1;
            if j == sub_arr.len(){
                res = true;
            }
        }
        else {
            i = i - j + 1;
            j = 0;
        }
    }

    println!("{}", res);
}