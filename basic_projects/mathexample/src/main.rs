
use std::collections::HashMap;

fn main() {

    let mut vec = vec![6,3,5,2,1,5];

    vec.sort();

    let mut sum = 0;
    let mut map = HashMap::new();
    let mut max_count = -1;
    let mut most = 0;

    for v in &vec {
        sum += v;
        let count = map.entry(v).or_insert(0);
        *count += 1;
        if *count > max_count {
            most = *v;
            max_count = *count;
        }
    }



    println!("avg:{}", sum / vec.len());
    println!("mid:{}", vec[vec.len() / 2]);
    println!("most:{}", most);

}
