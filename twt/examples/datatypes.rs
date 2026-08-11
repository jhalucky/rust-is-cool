fn main() {
    let x : &str = "We and We";
    println!("{}",x);

    let signed_int : i32 = -45;
    let unsigned_int : u32 = 78;

    let float : f32 = 973.89;
    let boolean : bool = true;

    println!("{}, {}, {}, {}",signed_int,unsigned_int,float,boolean);

    let strr: &str = "Go gulati Go!";
    println!("{}",strr);
    tup();
    arr();
}

fn tup() {
    let mut tup: (i32, i32, &str, bool) = (1, 3, "hello", true);
    tup.2 = "Gulati";
    println!("{:?}",tup);
    println!("{}",tup.1)
}

fn arr() {
    let mut arr: [i32; 5] = [10, 20, 30, 40, 50];
    arr[3] = 60;
    println!("{:?}",arr);
    println!("{}",arr[2]);
}