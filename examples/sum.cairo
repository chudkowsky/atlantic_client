%builtins output pedersen range_check bitwise

from starkware.cairo.common.alloc import alloc

func array_sum(arr: felt*, n) -> felt {
    if (n == 0) { 
        return 0;
    }
    let remainder = array_sum(arr=arr + 1, n=n - 1);
    return arr[0] + remainder;
}

func main{output_ptr, pedersen_ptr, range_check_ptr, bitwise_ptr}() {
    const ARRAY_SIZE = 3;
    let (ptr) = alloc();

    // populate array values
    assert [ptr] = 9;
    assert [ptr + 1] = 11;
    assert [ptr + 2] = 5;

    // Compute and check the sum
    let sum = array_sum(arr=ptr, n=ARRAY_SIZE);
    assert sum = 25;
    assert [output_ptr] = sum;

    let output_ptr = output_ptr + 1;

    return ();
}