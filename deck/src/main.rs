fn reverse<T: Copy>(arr: &mut [T]) {
    let mut left = 0;
    let mut right = arr.len()-1;
    while left < right {
        let temp = arr[left];
        arr[left] = arr[right];
        arr[right] = temp;
        left+=1;
        right-=1;
    }
}
fn main() {
    let mut arr = [1,2,3,4,5,6,7,8,9];
    reverse(&mut arr);
    println!("Hello, world! {:?}", arr);
}
