fu greet(person) {
    let greeting = format("Hello, ", person.name, ", nice to meet you.");
    print(greeting);
}

let person = {name: "Tim Apple"};

greet(person);