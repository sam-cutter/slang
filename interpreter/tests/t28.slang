let person = {
    name: "John Smith",
    age: 30,
    address: "house road"
};

let info = format(person.name, " (age ", person.age, ") lives at ", person.address);

print(info);