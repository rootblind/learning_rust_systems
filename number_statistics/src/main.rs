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
    let mut min_val = vec[0];
    let mut max_val = vec[0];
    let mut sum = 0;
    let mut odd_count = 0;
    let mut even_count = 0;

    for i in vec {
        if *i < min_val {
            min_val = *i;
        } else if *i > max_val {
            max_val = *i;
        }

        if i % 2 == 0 {
            even_count += 1;
        } else {
            odd_count += 1;
        }

        sum += i;
    }

    let avg: f32 = sum as f32 / vec.len() as f32;

    println!(
        "MIN: {}\nMAX: {}\nSUM: {}\nODD COUNT: {}\n EVEN COUNT: {}\nAVG: {}",
        min_val, max_val, sum, odd_count, even_count, avg
    );
}

fn main() {
    let vec = collection_generator();
    num_stats(&vec);
}
