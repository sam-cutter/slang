= Evaluation

== Overall effectiveness
I was able to successfully create an interpreter for a custom programming language, and implement multiple memory management methods. The programming language is general enough to solve many problems, and the memory management techniques have been implemented correctly.

== Evaluation of objectives
Below are the objectives I decided on for my project (sub-objectives are omitted), and my analysis of how successfully each one was met.

#table(
  columns: (1fr, 5fr),
  table.header([*Objective*], [*Success*]),
  [1], [
    All objectives in this section were met successfully, however some were more difficult to complete than others.

    Objective 1.4 was particularly tedious to implement, as every possible combination of type and operation had to be handled manually due to Rust's strict type system.

    Objective 1.5.1.5 was interesting grammatically, as the exponent operator is right associative, but most operators were left-associative.

    Objective 1.8.4.2 was slightly challenging to implement. I had to redesign how I stored environments and stack frames in order to allow every environment to link to the global scope.
  ],
  [2], [
    Again, all objectives were completely met in this section. There were some improvements that could be made, however.

    Objective 2.8.2 was implemented satisfactorily, however I could have added more advanced logic to figure out to a greater extent what the programmer was trying to do, and which mistake they made.

    Objective 2.11.2 was very challenging to implement, and required a lot of debugging to root out misconceptions I'd made about scope and object lifetimes.
  ],
  [3], [
    All objectives were successfully completed for this section.
  ],
  [4], [
    All objectives were met in this section.
    
    Objective 4.2.1 was met, however the user input experience could have been made better: currently users are not able to use the backspace key, or the arrow keys, meaning that they must enter a line of code perfectly the first time round.
  ]
)

== Memory management method analysis
At this point, I can answer the questions I set out in @analysis:statement. They are repeated below for convenience.

+ How does choice of memory management technique affect the execution time of a program?
+ How does choice of memory management technique affect the amount of memory used by the interpreter?
+ How does choice of memory management technique affect the speed at which objects are cleaned up?

To answer these questions, I wrote the following program:

```
fu fib(n) {
    let temp = {tag: n};
    x = {tag: temp};

    if n <= 1 { return n; }
    else { return fib(n - 1) + fib(n - 2); }
}

let x = {};

print(fib( [HARDCODED VALUE] ));
```

This program creates calculates the nth Fibonacci number, while also creating and reassigning objects frequently. I repeatedly ran this program, swapping out `[HARDCODED VALUE]` for the numbers 0 to 25, and recording the amount of time the program took to run. I did this for each memory management method.

#figure(image("../images/Execution time of fib(n) for different values of n.png"), caption: [Execution time of `fib(n)` for different values of `n`.])

As seen, naive allocation mode is slightly faster than reference counting, but only just. This means that reference counting performs very well--almost as well as if it had not been doing any memory management whatsoever. Garbage collection, however, is consistently slower than both other modes.

Taking a look into memory usage, for `fib(25)`, naive allocation mode allocated 306.5 megabytes, whereas reference counting and garbage collection allocated 75.08 and 75.09 megabytes respectively. This shows that using any memory management whatsoever leads to a drastic decrease in the amount of memory allocated.

Answering the third question required looking at the number of objects stored in the heap at any given time. The graphs below plot the number of objects in the heap against time. `fib(10)` has been called in each case.

#figure(image("../images/Heap objects count for fib(10) in na mode.png", width: 80%), caption: [Heap objects count for `fib(10)` in naive allocation mode.])

#figure(image("../images/Heap objects count for fib(10) in rc mode.png", width: 80%), caption: [Heap objects count for `fib(10)` in reference counting mode.])

#figure(image("../images/Heap objects count for fib(10) in gc mode.png", width: 80%), caption: [Heap objects count for `fib(10)` in garbage collection mode.])

There are a few things to note here. As expected, when using naive allocation the heap grows unchecked, never deallocating any objects. this leads to over 300 objects being allocated, which are never deallocated.

Garbage collection and reference counting, however, are able to rapidly clean up objects when they are no longer in use. When using reference counting, a maximum of 12 objects are stored in the heap at any one moment, and this rises to 21 with garbage collection. Comparing the two graphs highlights the advantage that reference counting holds over garbage collection: objects are deallocated the moment that they are no longer needed.

Within the specific `fib` program I wrote, the disparity between the two methods arises due to the line `x = {tag: temp};`. This line mutates the value of `x`, overwriting its previously stored value with a new object. In reference counting mode, the previously stored object has its reference count immediately decremented, deallocating it instantaneously, but in garbage collection mode the garbage collector is not called until the scope is exited (i.e. when the function returns). This leads to the previous value of `x` existing in the heap for far longer than is necessary.

The situation where garbage collection excels is in programs where reference counts are introduced. For reasons explained in @analysis:rc, reference counting is unable to clean up reference cycles. Naive allocation obviously doesn't even attempt to.

I wrote the following program to demonstrate this:

```
let i = 0;

while i < 100 {
    let a = {};
    let b = {};

    a.other = b;
    b.other = a;

    i = i + 1;
}
```

This repeats the following process 100 times: create two objects, and introduce a cycle by making both objects reference one another.

The graphs below plot the number of heap objects against time, for each memory management mode.

#figure(image("../images/Heap objects count for reference cycles in na mode.png", width: 80%), caption: [Heap objects count for reference cycling in naive allocation mode.])

#figure(image("../images/Heap objects count for reference cycles in rc mode.png", width: 80%), caption: [Heap objects count for reference cycling in reference counting mode.])

#figure(image("../images/Heap objects count for reference cycles in gc mode.png", width: 80%), caption: [Heap objects count for reference cycling in garbage collection mode.])

As seen, both reference counting and naive allocation do not clean up the reference cycles, leaving all objects allocated for the duration of the program even when they are obviously not needed anymore. Garbage collection is able to immediately clean up the reference cycles after each iteration, keeping the number of objects to 2 at a maximum.

== Expert feedback
I emailed Dr. Sleat to ask for some feedback on my interpreter, and he responded with the following:

#quote(block: true, quotes: true)[
Sam

Here is some feedback on Slang.

Overall I am very impressed. Within a short period of time you have created language that is reasonably functional and stable. This is a great start to allow you to explore how a stack based run-time system works and also to explore features of a language and how those impact the runtime. To have functions, objects as well as iteration and control flow, including with recursion, gives you a good start for further developing Slang. Personally (as an ex--C programmer) I appreciated the use of `;` to delimit statements and `{}` to define scope. In my view this is more explicit than the indentation-based approach of languages such as Python.

*Potential improvements to the language*

I have the following suggestions for areas where you might like to improve the language.

+ Checking type of parameters to functions. There is no type declaration in the function definition and this can cause runtime errors if the wrong type is passed. 
+ Beef up native functions to accept multiple parameters. This would be useful for `print()` and `format()` for example to avoid the coder having to either use lots of repeated `format()`s for multiple variable outputs. Indeed if print() allowed for multiple parameters (even if all string types) this would avoid having to use format().    
+ Casting to strings is related to point 2.
+ Having the ability to change the type of a variable is fine for dev/test languages but can cause untold damage in real-life programming environments. I would stick with strong variable type definitions.
+ I would relax the rules around type interchangeability of integers and floats. You should be able to write `x = 5 + 8.3;`
+ Arrays and/or lists would be nice.
+ Some additional flexibility around iteration/condition e.g. break/continue. 
+ How about including some built in functions that exposes the underlying GC state e.g. `memstats()` might return how much memory is currently allocated. This could be useful for debugging and understanding the GC abilities more thoroughly.

*Review of garbage collection*

I tried a couple of programs using recursion to assess the garbage collection. To be honest, I struggled to see how the `.csv` files that were generated added value to the programmer but, with further explanation and understanding on my part, I'm sure these would allow me to review the GC more accurately.

That being said, `slangtestA.txt` recursively calls a function to a depth decided by an input. I tested it to a depth of 414 before getting the error: `thread 'main' has overflowed its stack`. I'm not sure what the limits are here but 414 recursions didn't seem very many to me.

With `slangtestB.txt` I tried to create a cycle between two objects. This appeared to work but again, I'm not sure of the GC going on in the background. I made a couple of assumptions (a) that I can reference one object from the other - the language appeared to be 'loose' enough to allow me to do this where I assigned "a.pointertob = b" (b) that setting an object to 0 either calls some GC at that point or subsequently. I ran this to 10000 iterations without runtime failure.  I tested this with all three runtime options `gc`, `rc` and `na`. No problems.

Sam...this is, in my opinion, a very good piece of work. Many thanks for sharing it with me and please do shout if I can be of any further help.

Keep in touch. 

Kind regards

Doc
]

He attached the following `.txt` files:

*`slangtestA.txt`*
```
// ------------------------------------------------------------
// TEST A: Recursive allocation of temporary objects
// What it does:
//   - Recursively goes down to 0
//   - On each call, allocates a small object { n: depth }
//   - When the function returns, those objects should become unreachable
// ------------------------------------------------------------

fu allocDown(depth) {
  if depth <= 0 {
    return 0;
  }

  // Allocate a brand-new object on each recursive call
  let temp = { n: depth, tag: "temp" };

  // Make a tiny use of it - don't want any GC to collect unused objects
  if temp.n == -1 {
    print("Impossible");
  }

  // Don't like this, perhaps improve with more parameters to print statement and intelligent casting?
  let op = format("Recursing...", depth);
  print(op);

  // Recurse
  return allocDown(depth - 1);
}

let D = 0;
D = int(input("Depth ?"));

print("Starting allocDown...");
allocDown(D);
print("Finished allocDown.");
```

*`slangtestB.txt`*
```
// ------------------------------------------------------------
// TEST B: Create a tiny cycle and drop it in a loop
// What it does:
//   - Creates two objects that point at each other (a cycle)
//   - Drops the outside reference
//   - Repeats many times to see whether memory grows
// ------------------------------------------------------------

fu makeCycle() {
  let a = { name: "A" };
  let b = { name: "B" };

  // Create a cycle: A <-> B
  a.pointertob = b;
  b.pointertoa = a;

  return a;   // caller holds the only outside reference
}

let i = 0;
let tick = 1000;

print("Starting cycle test...");
while i < 100000 {
  let x = makeCycle();

  // Drop the last outside reference: the cycle is now "floating"
  x = 0;

  // Print progress every 1000 iterations 
  tick = tick - 1;
  if tick <= 0 {
    print(format("Made and dropped cycles: ", i));
    tick = 1000;
  }

  i = i + 1;
}
print("Finished cycle test.");
```

The feedback that Dr. Sleat provided was really helpful. Going point-by-point through his suggestions:

+ I would love to introduce a more robust type system, where the programmer can specify types of function arguments, function return types, variables etc. but it unfortunately wasn't feasible for the amount of time I had to write the interpreter.
+ This would be a very quick fix to implement for the `print` function.
+ Currently the programmer can cast any data type to a string via use of the `format` function, but this can be unwieldy. Adapting the type system for easier string concatenation and casting could be a solution.
+ Again, I would have wanted to construct a static type system if I'd had the time.
+ I'm not sure that I agree with Dr. Sleat on this point. It can be a bit frustrating having to cast an integer to a float before performing a floating point operation, but I think that this is justified by clarity over what type will be returned from the operation.
+ I definitely agree with introducing some more complex data structures, such as arrays and/or lists. This would allow many more programs to be written.
+ Again, adding additional statements such as `break` would be massively helpful.
+ Unfortunately this functionality does already exist, I just forgot to tell this to Dr. Sleat in my email to him. The programmer can access statistics about the program execution through global variables such as `HEAP_OBJECTS_COUNT` and `STACK_FRAMES_COUNT`.

The `slangtestA.txt` program was interesting, as Dr. Sleat discovered a recursion limit. This recursion limit was not implemented by me, and seems to be as a result of the operating system, or runtime. On my machine, I was able to reach 305 recursions, less than his 414.

Dr. Sleat wasn't able to analyse the `.csv` files to extract any useful information, so I ran `slangtestB.txt` and took a look through the resulting `.csv` file. It confirmed to me that garbage collection was indeed working correctly, with the number of objects stored on the heap flicking back and forth between 0 and 2.

== System improvements

=== Improving the type system
Currently, Slang is strongly typed (meaning that operations are type-checked before being performed) but dynamically typed, meaning that variables can change type throughout the course of the program, and functions do not have defined return or argument types. This leads to a lot of flexibility when writing code, which can be useful, but often leads to type errors occurring.

If I had the time, I would have liked to make Slang statically typed. The concept of static typing only really exists within compiled languages, because type checking occurs at compile-time in statically typed languages. Therefore, I would have to first write a compiler for Slang. This would be a very large project in of itself, and would likely involve interfacing with LLVM.

After writing a compiler for Slang, I would update the grammar and lexical grammar to ensure that variable definitions and function definitions have type annotations. From here, the process of checking whether a program is valid would be very similar to the process of executing the abstract syntax tree, but with no actual data values. By performing a post-order traversal of the tree, the types of the operands to each operation are checked. Any type errors are reported to the user, and the source code is not compiled if errors occur.

=== More advanced data structures
One of the most useful features that I could introduce would be a more advanced data structure such as an array or a list. This would allow many new types of languages to be written much more easily.

Firstly, I would add the required tokens, and write the lexer code to detect them. For list indexing, as well as list construction, I would likely use the left and right square brackets.

Next, I would need to add two new types of expression to the parser: the list indexing operation, and the list construction operation. List construction would be represented by the following new grammar rule:
```
list -> "[" (expression ("," expression)*)? "]"
```

An additional production would then be added to `primary`, which would be `| list`.

For list indexing, I would need to update the `call` grammar rule to the following:
```
call -> primary (
    "(" (expression ("," expression)*)? ")"
    | "." IDENTIFIER
    | "[" expression "]"
  )*
```

The parser would be updated to reflect these rules.

Next, I'd need to add new variants to `Expression` and `Value` to represent lists and their operations.

After this, the various operations (concatenation, appending, indexing etc.) would be implemented on the new list type.

=== Improving iteration constructs
I would like to add additional functionality for iteration, including `for`-loops, `break`-statements and `continue`-statements.

To implement this, I would introduce the following keywords: `for`, `break` and `continue`, and write the lexer code to recognise them.

In the parser, I would add two new types of statement:
```
breakStatement -> "break" ";"
continueStatement -> "continue" ";"
```

For-loops are slightly more complicated. The grammar rule would look as follows:
```
forLoop -> "for" "(" variableDeclaration ";" expression ";" expression ")" block
```

I'm using the syntax that is used in C\#, JavaScript and many other C-like languages.

Three new productions would then be added to `statement`, which would be `| breakStatement | continueStatement | forLoop`.

For each of these rules, the parser would be adapted to be able to parse them.

After, new `Statement` variants would be created, and the code to execute them written. `break`- and `continue`-statements simply alter the `ControlFlow` signal that is emitted from a statement execution. A `for`-loop initially runs its `variableDeclaration`, and continues to execute its `block` as long as the first `expression` evaluates to true, evaluating the second `expression` at the end of each loop.

=== Closures
I would like to make Slang functions truly first class. Currently, functions can be returned from other functions, passed as arguments to functions, assigned to variables, and used in expressions. However, functions are unable to _close_ around environments. This is best illustrated with the following example:

```
fu get_adder(a) {
  fu add(x) {
    return x + a;
  }

  return add;
}
```

Here, `get_adder` is a higher-order function that returns a _new_ function, whose job is to add `a` to its argument. It could be used as follows:

```
let add_5 = get_adder(5);

print(add_5(10));
```

Here, a function called `add_5` is created, which adds 5 to its argument. `15` is printed to the screen.

This code will not currently run in my interpreter, as when `add_5(10)` is ran, the line `return x + a;` is ran. `x` is replaced with `10`, but `a` is not defined within that scope. This is because the scope within `get_adder` has been lost.

To fix this, I would need to attach the environment that the function was defined in to the definition of the function, in order to access any variables that could have been used in that function.