fu test(x) {
    if x <= 1{
        return;
    } else if x <= 10 {
        return "small";
    } else {
        return true;
    }
}

print(test(100));
print(test(5));
print(test(0));