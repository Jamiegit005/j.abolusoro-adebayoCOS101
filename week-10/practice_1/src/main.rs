fn main() {

    let v = vec![101,250,330,400];
    //vector v owns the object in the heap

    //only a single variable owns the heap memory at any given time
    let _v2 = &v;
    //here two variables own heap value,
    //two pointers to the same content is not allowed in rust

}