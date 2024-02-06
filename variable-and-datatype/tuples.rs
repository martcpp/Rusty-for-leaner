// this are  collect of data that are stored in the array. tuple is the like a list in pythons.
// we can acess elements of the tuple by using the index of the element.

fn  main() {
    // this are mutable tuple
    let mut tuple = (1, 2, 3, 4, 5, 6, 7, 8, 9, 10);
    println!("{:#?}", tuple);

    
    // using the index of the element to access the element of the tuple
    tuple.2 = 100;
    println!("{}", tuple.0);
    
    // this are immutable tuple
    let student = ("John", "Doe", 25, 3.8, 'A');
    println!("{:#?}", student);

}