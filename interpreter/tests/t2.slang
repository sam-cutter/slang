fu test(x) {
    if x < 0 {
        return;
    } else if x < 10 {
        return x;
    } else {
        return "Very big.";
    }
}

print(test(0));
print(test(100));
print(test(-10));