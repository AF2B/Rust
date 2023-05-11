fn main() {
    let v = vec![1, 2, 3, 4, 5];
    let mut iter = v.iter(); // iter() returns an iterator. but what happen inside this "v.iter()"?
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    //let mut iter = v.iter_mut();
    println!("{:?}", iter.next()); // Some(&mut 1)
    println!("{:?}", iter.next()); // Some(&mut 2)

    let _just_match_example = match iter.next() {
        Some(x) => println!("Got {}", x),
        None => println!("No next element"),
    };

    let odd_numbers = vec![1, 3, 5, 7, 9];
    for i in odd_numbers.iter() {
        if i % 2 != 0 {
            println!("odd number: {}", i);
        }else {
            println!("even number: {}", i);
        }
    }

    let even_numbers = vec![2, 4, 6, 8, 10];

    for n in even_numbers.iter() {
        if n % 2 == 0 {
            println!("not all odd");
            break;
        }
    }

    let mut iter = vec![1, 2, 3, 4, 5].into_iter(); // into_iter() returns an iterator that takes ownership of the original collection
    println!("{:?}", iter.next()); // Some(1)
    println!("{:?}", iter.next()); // Some(2)
    println!("{:?}", iter.next()); // Some(3)

    let vector_example_iter = vec![1, 2, 3, 4, 5];
    for vector_example in vector_example_iter.iter() {
        println!("vector_example: {}", vector_example + 1);
    }

    // iter() iters upside a collection without consuming it
    // into_iter() iters upside a collection consuming it
    // iter_mut() iters upside a collection mutably
}
