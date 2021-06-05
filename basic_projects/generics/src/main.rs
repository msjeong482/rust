
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &number in list.iter() {
        if number > largest {
            largest = number
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("the largest:{}", result);

    let number_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&number_list);
    println!("the largest:{}", result);
}
