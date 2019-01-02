use std::io;
mod quick;
use crate::quick::quicksort;
mod selection;
use crate::selection::selcsort;
mod bucket;
use crate::bucket::bucketsort;

fn main() {
    let mut n = String::new();
    io::stdin()
        .read_line(&mut n)
        .expect("error in reading");
    let n: usize = n.trim().parse().expect("invalid input");
    println!("Length of the array:{:?}\n",n);

    let mut array: Vec<usize> = vec![];

    for _x in 1..(n+1) {
        let mut i = String::new();
        io::stdin()
            .read_line(&mut i)
            .expect("error in reading");
        let i: usize = i.trim().parse().expect("invalid input");
        array.push(i as usize);
    }

    let mut array1 = array.clone();
    let array2 = array.clone();
    println!("Original:{:?} ",array);
    quicksort(&mut array);
    println!("Quicksorted:{:?}\n",array);

    println!("Original:{:?} ",array1);
    selcsort(&mut array1);
    println!("Selection sorted:{:?}\n",array1);

    println!("Original:{:?} ",array2);
    print!("Bucket Sorted:{:?}",bucketsort(array2,|int| int/10));
}
