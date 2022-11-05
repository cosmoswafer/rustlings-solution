// move_semantics2.rs
// Make me compile without changing line 13 or moving line 10!
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.


fn main() {
    let vec0 = Vec::new();

    let vec2 = vec0.clone();
    let mut vec1 = fill_vec1(vec0.clone());

    let mut vec1 = fill_vec2(&vec0);

    // Method 3
    //let vec0 = Vec::new();
    //fill_vec3(&mut vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec1(vec: Vec<i32>) -> Vec<i32> {
    // Method 1
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}

fn fill_vec2(vec: &Vec<i32>) -> Vec<i32> {
    // Method 2
    let mut vec2 = vec.clone();

    vec2.push(22);
    vec2.push(44);
    vec2.push(66);

    vec2
}

fn fill_vec3(vec: &mut Vec<i32>) {
    // Method 3
    //let mut vec2 = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    //vec2
}
