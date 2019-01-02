pub fn choose_pivot<T: Ord>(slice : &[T]) -> usize {
    let (ismall, imid, ibig) = (0, slice.len()/2,slice.len()-1);
    //if slice[ibig] < slice[ismall] {swap(&mut ibig, &mut ismall);}
    if slice[imid] <= slice[ismall] {ismall}
    else if slice[ibig] <= slice[imid] {ibig}
    else{imid}
}

pub fn partition<T: Ord>(slice: &mut [T],pivot: usize) -> usize{
    let mxix = slice.len() - 1;
    slice.swap(pivot,mxix);
    let (mut left, mut right) = (0,mxix -1);

    while left < right {
        if slice[left] <= slice[mxix] {left += 1;}
        else if slice[right] >= slice[mxix] {right -= 1;}
        else {
            slice.swap(left,right);
            left += 1;
            right -= 1;
        }
    }

        if left> right {
            slice.swap(left,mxix);
            return left;
        }

        if slice[left] >= slice[mxix] {
            slice.swap(left,mxix);
            return left;
        } else if slice[left] <= slice[mxix] {
            slice.swap(left +1,mxix);
            return left+1;
        }
        panic!("This should be unreachable. Indices:{},{} / {}",left,right,mxix);
}

pub fn quicksort<T: Ord>(slice: &mut [T] ) {
    if slice.len() <= 1 {return;}
    else if slice.len() == 2{
        if slice[0] >= slice[1] {slice.swap(0,1);}
        return;
    }

    let pivot = choose_pivot(slice);
    let pivot  = partition(slice,pivot);
    let (leftslice,rightslice) = slice.split_at_mut(pivot as usize);
    let rightslice = &mut rightslice[1..];

    quicksort(leftslice);
    quicksort(rightslice);
}
