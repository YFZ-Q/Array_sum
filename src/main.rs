fn main() {
    //u32数据范围0～4294967295

    //溢出情况，返回None
    let list:Vec<u32> = vec![4294967294, 2];

    //非溢出情况，返回Some(数值)
    let list_no:Vec<u32> = vec![10, 2, 15];

    let res:Option<u32> = list_plus(&list);
    println!("{:?}", res);

    let res_no:Option<u32> = list_plus(&list_no);
    println!("{:?}", res_no);
    
}

fn list_plus(list:&[u32]) -> Option<u32>{
    let mut res = 0;
    for i in list.iter() {
        //比较集合中此时要加的数值与res和u32类型的边界数值的差值，若是小于差值，则可以继续求和，否则返回None
        let dif = 4294967295 - res;
        if &dif >= i {
            res = res + i;
        } else {
            return None;
        }
    }
    return Some(res);
}