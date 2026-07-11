fn bubble_sort(arr :&mut [i32]) {
    let len = arr.len();
    if len < 2 {
        return;
    }
    for pass in 0 .. len - 1 {
        let mut swapped = false;
        for i in 0 .. len - pass - 1 {
            if arr[i] > arr[i + 1] {
                arr.swap(i, i + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}

fn main() {
    let mut arr = [1,3,4,2,5,8,6,7,0,9];
    bubble_sort(&mut arr);
    print!("{:?}", arr);
}
