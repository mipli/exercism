pub fn map<T, U>(input: Vec<T>, mut function: impl FnMut(T) -> U) -> Vec<U> {
    let mut acc = vec![];
    for val in input {
        acc.push(function(val));
    }
    return acc;
}
