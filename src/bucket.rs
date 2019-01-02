struct Bucket<H, V> {
    hash: H,
    values: Vec<V>,
}

impl<H, V> Bucket<H, V> {
    fn new(hash: H, value: V) -> Bucket<H, V> {
        Bucket {
            hash: hash,
            values: vec![value],
        }
    }
}

pub fn bucketsort<T, F, H>(values: Vec<T>, hasher: F) -> Vec<T>
    where T: Ord,
          F: Fn(&T) -> H,
          H: Ord
{
    let mut buckets: Vec<Bucket<H, T>> = Vec::new();

    for value in values {
        let hash = hasher(&value);
        match buckets.binary_search_by(|bucket| bucket.hash.cmp(&hash)) {
            Ok(index) => buckets[index].values.push(value),
            Err(index) => buckets.insert(index, Bucket::new(hash, value)),
        }
    }

    buckets.into_iter().flat_map(|mut bucket| {
        bucket.values.sort();
        bucket.values
    }).collect()
}
