fn main() {
    let mut b = vec![04,54,34, 43];
    &mut b.insert(1, 34);
    println!("{:?} Hello, world!", &b[2..]);

    let gino = vec![432,454,656,35345];
    for i in gino{
        b.insert(1, i);
        println!("hello, {:?}", b);
        //pagnone(20);
                let bivvo: Vec<i32> = (2..200).collect(); //.collect() collects the range into a container, 
                //but you need to specify what kind of container you want, otherwise it doesn't know what type to return
            for  pippo in bivvo{
            println!("Hello pippo numero {:?}", pippo);
            }
    }
}

fn pagnone(a:u32) {
    let b = vec![2..a];
    //for i in b.iter().next() {
    //    println!("Hello pippo numero {:?}", &i);
    //}
    println!("Hello pippo numero {:?}", &b);
}