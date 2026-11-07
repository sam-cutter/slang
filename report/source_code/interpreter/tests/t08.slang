// Equal to
print(1 == 1);
print(1 == 0);

print(1.0 == 1.0);
print(1.0 == 0.0);

print("Hello" == "Hello");
print("Hello" == "World");

let x = {label: "test"};
let y = x;
print(x == y);
print(x == {label: "test"});
print(x == {label: "test", label2: "another test"});

print(true == true);
print(true == false);

// Not equal to
print(1 != 1);
print(1 != 0);

print(1.0 != 1.0);
print(1.0 != 0.0);

print("Hello" != "Hello");
print("Hello" != "World");

let x = {label: "test"};
let y = x;
print(x != y);
print(x != {label: "test"});
print(x != {label: "test", label2: "another test"});

print(true != true);
print(true != false);