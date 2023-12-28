fn main() {
    let v = vec![10,20,30];
    // vector v owns the object in heap

    let _v2 = &v; // moves ownership to v2

    display(_v2.clone());
    //v2 is moved to display and v2 is Invalidated

    println!("In main{:?}",_v2);
    //v2 is no longwe usable here
}

fn display(v:Vec<i32>){
    println!("inside display{:?}",v);

}
