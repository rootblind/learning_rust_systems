use rand::Rng;

fn collection_generator() -> Vec<i32> {
    let mut rng = rand::rng();
    let size = rng.random_range(1..=100);
    let mut vec = vec![size];
    for _i in 0..size {
        vec.push(rng.random_range(-1000..=1000));
    }

    return vec;
}

fn num_stats(vec: &Vec<i32>) {
    if vec.len() == 0 {
        panic!("The vector provided is empty!");
    }

    let min_val = *vec.iter().min().unwrap();
    let max_val = *vec.iter().max().unwrap();
    let sum: i32 = vec.iter().sum();
    let odd_count: i32 = vec.iter().filter(|&x| x % 2 == 1).count() as i32;
    let even_count = vec.iter().filter(|&x| x % 2 == 0).count() as i32;
    let avg: f32 = sum as f32 / vec.len() as f32;

    println!(
        "MIN: {}\nMAX: {}\nSUM: {}\nODD COUNT: {}\nEVEN COUNT: {}\nAVG: {}",
        min_val, max_val, sum, odd_count, even_count, avg
    );
}

fn main() {
    let vec = collection_generator();
    num_stats(&vec);
}
