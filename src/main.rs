use exercise::structs::heap::MinHeap;

fn main(){
    let mut heap = MinHeap::from(vec![90, 87, 61, 69, 31, 9, 23, 11]);
    let mut sortArr = Vec::<usize>::new();

    while heap.size() > 0 {
        sortArr.push(heap.pop())
    }

    println!("let me kan kan {:?}", sortArr);

    assert_eq!(sortArr, vec![9, 11, 23, 31, 61, 69, 87, 90])
}