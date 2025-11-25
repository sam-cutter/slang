// This is a line comment.

/*
This is a block comment. Anything in here will be ignored.

print("This will not be run.");
*/

if true {
    // Blocks are enclosed in braces.
}

// Parameter lists to subroutines are enclosed in parenthesis.
fu greet(firstName, lastName, age) {
    // Statements are terminated with semicolons.
    print(format("Hello ", firstName, " ", lastName, ". You are ", age, " years old."));
}

// Arithmetic expressions are enclosed in infix notation.
let x = 1 + 2 * 3 - 4;