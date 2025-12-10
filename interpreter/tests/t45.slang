fu create_object() {
    let x = {};
    return x;
}

let i = 0;

while i < 100 {
    i = i + 1;
    let x = create_object();
}