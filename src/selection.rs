pub fn selcsort<T: Ord>(slice: &mut [T]){
    if slice.len() == 1 || slice.len() == 0 {return;}
    let mut min = 0;
    for x in 1..slice.len(){
        if slice[x] <= slice[min]{min = x;}
    }
    slice.swap(0,min);
    selcsort(&mut slice[1..]);
}
