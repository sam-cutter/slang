let name = input("What is your name? ");

print(format("Nice to meet you, ", name));


let continue = true;

while continue {
    if input("Do you want to continue? (y/n) ") != "y" {
        continue = false;
    }
}

print("Done.");