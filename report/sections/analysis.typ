#import "../config.typ": question

= Analysis

== Statement of the investigation <analysis:statement>
Most interpreted programming languages now offer automatic memory management, which moves the burden from the user to the runtime. There are various solutions for managing memory, including reference counting, garbage collection and ownership, however each have their tradeoffs with respect to runtime performance, implementation complexity and learning curve for the developer. This report aims to investigate different memory management techniques, in order to evaluate their relative performance within an interpreted environment.

My report will answer the following questions:
+ How does choice of memory management technique affect the execution time of a program?
+ How does choice of memory management technique affect the amount of memory used by the interpreter?
+ How does choice of memory management technique affect the speed at which objects are cleaned up?

== Background
A translator is a program which translates a sequence of instructions--the _source code_--from one language to another. Typically, the translation is from a human readable, _higher level_ language, into a language which can be understood by machines--a _low level_ language. There are two main types of translator: compilers and interpreters.

Compilers translate the source code in one go, producing a file which contains the entirety of the translated code, which can then be executed independently.

An interpreter, however, reads the source code, translating then immediately executing one instruction at a time, without producing any translated output.

When an interpreter executes a sequence of instructions, it is responsible for managing the memory it uses. Since the computer doesn't have infinite memory, it's important to make efficient use of what's available, so the interpreter must make correct and useful judgements about which information is still in use (or could be used in the future) by the program.

== The expert
I reached out to Dr. Philip M. Sleat, who taught me further maths and computer science at GCSE level, for assistance with this investigation. He has a PhD in Computer Science, and had a career as a software engineer before moving into teaching.

== Initial research

=== How interpreters work
In order to gain an understanding of how interpreters work, I read the first four chapters of _Compilers: Principles, Techniques, and Tools, 2nd edition_ @dragon. This gave me an understanding of formal grammars, lexical analysis and parsing.

==== Lexical analysis <analysis:initial:how:lex>
The lexical analyser works at the front line of the interpreter, reading the raw source code which the user has written, and converting this information into a much cleaner, easier to work with format: a sequence of tokens. A token represents the smallest meaningful unit of a language. For example, consider the following code:

#align(center, `let name = "Sam";`)

The above code assigns the string "Sam" to the identifier `name`. Running this through a lexical analyser would produce something like:

#align(center, image("../images/e1-tokens.png", width: 70%))

Here, each box represents a single token, and the text inside each box represents the type of token. In some cases, for example with `identifier`, additional information needs to be stored, in which case the data is shown in parenthesis.

We see that no information--barring whitespace--has been lost from the original source code. The information has simply been converted into a format which is easier for the next stage of the interpreter to work with.

In order to produce this sequence of tokens, the lexical analyser must understand the _lexical grammar_ of the language. The lexical grammar describes exactly which sequences of characters map to which tokens.

Let's imagine a simple language which supports basic arithmetic and logical operations. If, while scanning through the source code, we encounter a `+` character, we immediately know that this must be a `Plus` token, as no other token begins with the `+` character. However, if instead we encountered an `=` character, we don't immediately know which token is being described: it could either be a single equals sign (the `Equal` token), which may represent assigning a value to a variable, or it could be two equals signs right next to one another (`DoubleEqual`), which would then be describing an equality test. In order to determine which token we have, we must look ahead into the source code. In this example, another equals sign would confirm that we are consuming a `DoubleEqual` token, and any other character (or no next character at all) would mean that we have an `Equal` token.

The distance into the source code at which we may need to peek is called the _lookahead_. Most languages only need one or two characters of lookahead, which is fortunate as lexical analysers tend to slow down as lookahead increases. For this reason, I will ensure that my language will only require two characters of lookahead (see Objective 2.3).

More complex tokens, for example numbers, are slightly more complex to read, but it ultimately comes down to consuming characters, and occasionally peeking a number of characters ahead to figure out what's going on.

==== Parsing
The parser takes in the sequence of tokens produced by the lexical analyser, and produces a data structure which represents the semantics of the user's program. This data structure should take into account rules to do with operator precedence and associativity.

Consider the following source code:

#align(center, `1 + 2 * 3 - 4`)

As per @analysis:initial:how:lex, lexical analysis analysis would produce the following:

#align(center, image("../images/e2-tokens.png", width: 70%))

If a human were to evaluate the code, they would likely group the separate parts of the expression, applying the rules of operator precedence they learnt in primary school. They would first evaluate the central brackets: `1 + 6 - 4`, before evaluating from left to right: `7 - 4`, then `3`.

This method of nesting and evaluating sub-expressions lends itself well to storing the program in a tree-like data structure, usually known as an _abstract syntax tree_. In our example, the abstract syntax tree produced would look something like this:

#figure(image("../images/e2-tree.png", width: 40%), caption: "An expression tree.") <expression_tree>

In this diagram, the internal nodes represent operations, and the leaf nodes represent operands. Once again, the information stored by the tokens has not been changed, simply rearranged into a more useful format.

The process of parsing is slightly more complicated, but in many ways similar to lexical analysis. The general idea is to attempt to match sequences of tokens against pre-described patterns. The set of valid patterns which make up a language is known as its _grammar_.

A grammar contains a set of _non-terminals_, and for each non-terminal, a set of _productions_. The productions are the various 'patterns' which are associated with that non-terminal, and may contain a mixture of _terminals_ (actual tokens) and other non-terminals. The parser builds up the abstract syntax tree by attempting to parse a designated 'start' non-terminal, and recursively replacing non-terminals with associated productions, depending on the contents of the source code. This type of parser is called a _recursive descent parser_. I plan on using this type of parser for my interpreter (see Objective 2.5), as it is relatively simple to implement but very powerful.

==== Evaluation
Once an abstract syntax tree (AST) has been produced, it can be directly evaluated. As each internal node represents an operation, in order to evaluate that internal node, its children must first be evaluated. This means that a post-order traversal is appropriate for evaluating the AST.

Once the children for an internal node have been evaluated, the operation which that node describes can be carried out. It's at this point that any type checking should occur, to ensure that operations which are poorly defined (e.g. adding a string to an integer) will not be carried out. I plan on performing type checks at runtime to prevent strange runtime errors appearing (see Objective 1.4). It's also important to check for other invalid operations such as division by zero at this stage. Once the types have been checked, the operation is performed on the values of the children, and the result returned to the parent operation.

Some operations, such as accessing variables, require access to the _environment_. The environment is a collection of _scopes_, arranged in a stack, where each scope contains variable and function definitions. A new scope is added whenever a block (e.g. an if-statement, while-loop etc.) is entered, and the scope is popped off the environment stack when that block is exited. In order to get the value of a variable, the most recent scope is first checked, then its parent, and then the program carries on iterating through the scopes until the original one is reached (or until the variable in question is found).

The environment stack is different to the call stack, in that each call stack frame has its own environment stack contained within it. However, in order to facilitate global variables, the global variable scope exists externally to the call stack.

I plan on using the post-order traversal method to evaluate abstract syntax trees produced by my parser, as it is simple to implement, and is easy to understand conceptually (see Objective 2.6).

=== Memory management techniques
Any data stored on the heap must be managed, in order to prevent the memory which a program uses from growing unchecked. These are some of the methods commonly used for managing memory.

==== Naive allocation
What is sometimes known as the naive method is not used within any serious programming languages, as it practically useless. Allocated memory is never deallocated, even when obvious that the information it stores will never be needed again. As a consequence, running programs will use huge amounts of memory, the vast majority of which is wasted. I will implement this as one of the memory management options, in order to provide a 'control' for the benchmarks (Objective 2.11.1).

==== Reference counting <analysis:rc>
Reference counting is a technique which keeps track of how many parts of the program rely on (reference) a piece of information, deallocating the memory once the number of references drops to zero. This technique is generally very effective, as objects' memory is deallocated as soon as the object is no longer needed. However, any situation in which an object references itself, directly or otherwise, cannot be handled by the reference counter.

#figure(image("../images/reference_cycle.svg", width: 60%), caption: "A reference cycle.") <reference_cycle>

Consider @reference_cycle. Let's imagine that Object A was deleted. Object B's reference count would therefore be decreased by one, however due to the reference from Object E, its count would not fall to zero, meaning that Object B would not be deleted. Therefore, the reference from Object B to C would not be dropped, and neither would all subsequent references. Objects B, C and D and E would not be deleted, despite existing entirely isolated from the rest of the program, completely unreachable.

In my solution, I will implement this as one of my memory management techniques (see Objective 2.11.2), as it is relatively simple to implement, while being generally fast and effective. Furthermore, my programming language will need to support basic objects which can reference one another (see Objective 1.7), in order to demonstrate the limitations of reference counting.

==== Mark-and-sweep garbage collection
Mark-and-sweep garbage collection maintains a graph of references between objects, and frequently traverses this graph, _marking_ all objects which can be reached, and then _sweeping_ through the graph, dropping all objects which are unreachable.

In the example given in @reference_cycle, as soon as the reference from Object A to B is dropped, then Object B will not be reachable, meaning that Objects B, C, D and E will be deleted.

This technique tends to be relatively robust, however frequent traversal of the reference graph may slow down the execution of the program, so much so that the interpreter can sometimes be forced forced to temporarily stop execution of the user's code so that it can run the garbage collector and clean up its memory (these moments are known as 'stop the world moments'). However, the time cost can be justified by the memory saved, especially in programs where there tend to be many reference cycles.

I will implement mark-and-sweep garbage collection in my solution, to demonstrate a more widely used memory management method (see Objective 2.11.3). I will benchmark programs written in my language to see how much the running of the garbage collector itself impacts the performance of the interpreter, as compared to other memory management methods (see Objective 3).

=== Existing interpreted languages

==== Python
Python is often the language which people learn first, due to the simplicity of its syntax, and the huge community behind it. It's not just used by beginners, though. Python is one of the most used languages for machine learning and data analysis.

Python is a multi-paradigm language, supporting object-oriented programming, imperative programming, as well as aspects of functional programming. It's dynamically (but strongly) typed, with the types of operands being checked at runtime to ensure undefined behaviour doesn't occur.

#block(breakable: false)[
```py
def fibonacci(n):
  if n <= 1:
    return 1
  
  return fibonacci(n - 2) + fibonacci(n - 1)
```]

Shown above is a recursive implementation of getting the nth Fibonacci number in Python. As seen, variables and function definitions are not annotated with types. Unlike languages in the C family, Python does not use any statement terminators such as a semicolon, and it also does not enclose blocks with curly braces.

#block(breakable: false)[
```py
def bubble_sort(items):
  for i in range(0, len(items)):
    swaps_made = False

    for j in range (0, len(items) - i - 1):
      if items[j] > items[j + 1]:
        (items[j], items[j + 1]) = (items[j + 1], items[j])
        swaps_made = True

    if not swaps_made:
      break

  return items
```]

Above is another example of some Python code, this time a bubble sort. Control flow is shown through indentation, which makes for fairly elegant code, however this syntax is disliked by many.

Under the hood, Python is first compiled to bytecode, which is then ran in a virtual machine. Python uses a combination of reference counting and garbage collection to manage its memory, primarily employing reference counting, but with some cycle detection features to catch what a typical reference counter cannot.

#heading("Advantages", numbering: none, depth: 5)
- Syntax is very beginner friendly and readable.
- Dynamic type system provides a lot of flexibility, and leads to more concise code due to lack of type annotations.
- Automatic memory management removes a burden from the programmer.

#heading("Disadvantages", numbering: none, depth: 5)
- Relatively slow execution times.
- Dynamic typing can lead to unpredictable runtime errors.

As my language will also be dynamically typed and interpreted, it will experience the same issues with runtime errors and speed that Python does. Performance issues introduced by the garbage collector may also occur within my language, but I will allow the programmer to choose their desired method of memory management (see Objective 4.1), so the power will lie with them to decide which memory management method would be the most appropriate. I'd like to mimic the level of flexibility and ease of use that Python provides, by not forcing the user to annotate their variables and functions with types (see Objective 1.3.1), and allowing the types of variables to change throughout the execution of the program (see Objective 1.3.2). I also like the fact that although Python is dynamically typed, it is strongly typed, meaning that it ensures poorly defined operations (such as adding a Boolean and an integer) will not be performed, and an error will instead be thrown at runtime. I will implement these runtime type checks within my interpreter (see Objective 1.4).

==== JavaScript
JavaScript is the language which powers the vast majority of websites on the Internet. Virtually all browsers have a JavaScript engine, whose job is to interpret scripts embedded into the HTML of webpages. This allows developers to create complex web applications, which can fetch data from databases, change the style of the website dynamically, and run complex algorithms.

JavaScript is dynamically typed and, just like Python, supports multiple programming paradigms.

#block(breakable: false)[
```javascript
function fibonacci(n) {
    if (n <= 1) {
        return 1;
    }
    
    return fibonacci(n - 2) + fibonacci(n - 1);
}
```]

Here, the Fibonacci function is implemented in JavaScript. Notice the C-like syntax, with curly braces and semicolons, and parenthesis surrounding `if` statement conditions.

#block(breakable: false)[
```javascript
function bubble_sort(items) {
  for (let i = 0; i < items.length; i++) {
    let swaps_made = false;
    
    for (let j = 0; j < items.length - i - 1; j++) {
      if (items[j] > items[j + 1]) {
        let temp = items[j];
        items[j] = items[j + 1];
        items[j + 1] = temp;
        
        swaps_made = true;
      }
    }

    if (!swaps_made) {
      break;
    }
  }

  return items;
}
```]

The bubble sort algorithm is implemented in JavaScript above. Here, it becomes clear that JavaScript code can be more verbose than Python code, but control flow is often slightly more clear, due to the use of curly braces surrounding blocks of code.

JavaScript engines typically use a combination of interpreting and compiling, known as just-in-time compilation. Initially, the JavaScript program is interpreted, but as it runs, the engine analyses the code, and will compile sections of the code--such as frequently called functions--to achieve speedups. Since the engine has access to more information than a standalone compiler would, programs can often reach faster execution times using this method.

JavaScript uses garbage collection to clean up memory no longer in use, specifically mark-and-sweep garbage collection, which 'marks' all reachable objects and then removes all objects which are unmarked.

#heading("Advantages", numbering: none, depth: 5)
- The use of just-in-time compilation achieves much faster speeds than purely interpreting the source code.
- Similarly to Python, the lack of type annotations leads to more concise code.
- Robust garbage collection allows the programmer to focus on the logic of their program rather than managing memory.

#heading("Disadvantages", numbering: none, depth: 5)
- For some types, operations are defined in a confusing manner, for example between a string and an integer, the `+` operator treats the number as a string and performs string concatenation, whereas `-` attempts to coerce the string into an integer, then perform the arithmetic subtraction.
- Dynamic typing can lead to difficult-to-trace runtime errors, especially when the `undefined` value is introduced.

Just-in-time compilation would be too complex to implement for this project, so I will stick to purely walking the abstract syntax tree produced by the parser (see Objective 2.6). The memory management that JavaScript provides is generally very effective, so I will provide garbage collection as one of the memory management options (see Objective 2.11.3). I dislike the fact that some operations are poorly defined, and are performed anyway without reporting an error, so I will ensure that my language is strongly typed (see Objective 1.4). I prefer the syntax of JavaScript to Python, as it's often more clear how the code works, so I will base the syntax of my language around that of the C family's (see Objective 1.2).

=== First interview
#question(
  [
    Which memory management techniques would be both feasible and valuable to implement and investigate?
  ],
  [
    // It would be interesting to throw into the mix some kind of manual memory allocation, the equivalent of _malloc_ and _free_ in C, because that's very different to the tools that, for example, Python provides with automatic garbage collection. Particularly in my day, programming in C, if you wanted to push to a list you had to write your own code to allocate the memory for the new item, then add a pointer to it.
   With garbage collection, you can have _stop the world moments_, where the interpreter is no longer running the user's code, but it's running all this background stuff to try and clean up its memory. However, you don't get the hassle of memory leaks. You could also think about reference counting, which doesn't deal with reference cycles or self-referential objects, so you'd have to watch out for that. I would stick with mark-and-sweep, reference counting and perhaps seeing what happens when memory is left unmanaged. That sounds quite ambitious to me, but there we go. As a potential extension to the project, you could implement manual memory management. Manual memory management gives you that element of direct control, but also the hassle of problems with memory leaks and things like that. The advantage is that you can often get much faster, more predictable execution times. 
  ],
  [
    Dr. Sleat suggested that I should implement mark and sweep garbage collection (see Objective 2.11.3), reference counting (see Objective 2.11.2), and also naive allocation, where no memory management occurs (see Objective 2.11.1). I will most likely not implement manual memory management, as I think it will be beyond the scope of this project.

    He also mentioned some of the advantages and pitfalls of the various techniques. Garbage collectors can cause _stop the world moments_, where execution of the user's program has to temporarily be stopped while the garbage collector cleans up memory, to avoid running out of memory. This could cause issues in time-critical systems. With manual memory allocation, it is very easy to accidentally create bugs which leak memory, since the control of memory is entirely down to the user. However, this is a double edged sword, as code with manual memory management may have more predictable execution times (there is no garbage collector to cause stop the world moments). Dr. Sleat also mentioned the issues with reference counting, namely the fact that it can't deal with reference cycles.
  ]
)

#question(
  [
    What is the minimum feature set that the language should have in order to be able to evaluate each technique fully?
  ],
  [
    You're going to need to have some element of global variables, whose state persists for the entire duration of the execution of the code and which you don't want to be cleared up in any garbage collection or memory management. I would suggest that you probably need some functions, in order to create some local variables within those functions, and maybe handle some arguments that are going to be stored on a stack or something like that. And if you've got some functions, then you can probably implement some very simple recursion for something like Fibonacci or factorial, nothing too complicated. And then obviously have some sort of basic object which is able to reference objects in order to test reference counting and reference cycling.
  ],
  [
    Dr. Sleat told me that I would need a range of features. The first is global variables (see Objective 1.8.4.2), which should not be cleaned up in memory management. Next is functions (see Objective 1.10), which should have local variables and arguments. Functions should support recursion (see Objective 1.10.5). There should also be some objects (see Objective 1.7), which are able to reference other objects (and themselves), so that I can test reference cycle handling.
  ]
)

#question(
  [
    Which paradigm (object oriented, imperative etc.) would you recommend for my language, and why?
  ],
  [
    I would try to keep it as simple as possible. You've got a lot to tackle and probably an imperative language is going to be your best option, with very basic objects.
  ],
  [
    From the answer to this question, it's clear that I should stick to an imperative paradigm (see Objective 1.1). However, I will still need implement the ability to have objects. They will not be advanced objects, however.
  ]
)

#question(
  [
    Would making my language statically typed be feasible? If so, would it be relevant to this project?
  ],
  [
    With dynamic typing, you can use a piece of memory however the code wants to use that memory at that moment in time. So when you set up a variable, it's really just pointing to a lump of memory that's going to be used. In terms of the runtime system, that is probably going to be the easiest because you're just allocating a bit of memory and then using it as the bit of imperative code sees fit. From a functionality, ease of use, purism of the language point of view, it is horrible. I mean in Python you can set a variable to an integer, and in the very next line you can set it to a string. But it's probably the simplest. If you have static casting of your data types, then you're going to have to put a bit more effort into making that work. So I would keep it as simple as possible if I were you.
  ],
  [
    Dr. Sleat suggested that, once again, I should keep things simple. For that reason, I'll keep my language dynamically typed (see Objective 1.3), but perform runtime checks to check that operations are type safe (see Objective 1.4).
  ]
)

#question(
  [
    As someone who has programmed in both C and Python, what would you say are the relative advantages and disadvantages of both, with respect to performance, ease of use and available features?],
  [
    C is obviously fast, and very low level, very prone to error, lots of memory leaks all over the place if you're not careful. Python is absolutely great. If you've got a list in Python and you want to add something to the end of it, it's as easy as that. But as you might remember me saying in computer science lessons, it makes for sloppy coding and makes for not very intelligent programmers because they don't know how to do the low level stuff, which is what I think they should know. It depends what the application is, of course, because if you're coding something where you want speed, then C might be the way to go. If you're just knocking up some quick applications and you're not very experienced then Python may be the way to go.
  ],
  [
    From the answer to this question, it's clear that Python sacrifices low level control and speed for ease of use and flexibility. Since my language is only going to be used for basic testing, it should be fine for it to be closer, in terms of ease of use, to Python. The syntax of my language will be closer to that of C (see Objective 1.2), as Python's indentation style can sometimes be ambiguous.
  ]
)

#question(
  [
    Once completed, what kinds of algorithms should I implement and benchmark in my language to measure the performance of each technique?
  ],
  [
    // I would suggest doing something with lists: lots of appending, deleting the middle parts of those lists and the ends of those lists.
    If your programming language can cope with it, doing some recursive calls of a very simple function like a factorial. To see the difference between garbage collection and reference counting, it will also be necessary to implement some programs which introduce reference cycles.
  ],
  [
    In my programming language, I'll implement a recursive algorithm, such as the Fibonacci function (see Objective 3.1.1). I'll also try to introduce some reference cycles between objects (see Objective 3.1.2) so that I can differentiate between reference counting and garbage collection.
  ]
)

#question(
  [
    How should I measure the performance of the memory management techniques--which statistics should be tracked?
  ],
  [
    Obviously, runtime memory usage, and make sure to track the number of objects at any one time. That will help to show the differences between reference counting and garbage collection. You should also track the time that programs take to execute, that kind of thing.
    // If you're using mark and sweep, you should measure the the timing impacts of the garbage collection. It could also be useful measuring any memory leaks. You can have a look at the number of allocations and deallocations over time, that kind of thing.
  ],
  [
    As part of my investigation, I'll measure the memory usage of the interpreter (see Objective 3.2.4), the time that various programs take to execute (see Objective 3.2.1). I may also measure the number of objects at any one moment (see Objective 3.2.2).
  ]
)

#question(
  [
    What are some common pitfalls which programming languages fall into?
  ],
  [
    Memory leaks introduced by the runtime system. It does happen. And again it's it's the impact of that garbage collection. If that's too great, then the code could become too slow. How much does it expose to the programmer? How much do you want to expose to the programmer and how much damage can they cause by doing the wrong thing? That clarity of how the memory is handled, I think more than anything.
  ],
  [
    I learnt here that the runtime system can actually introduce more problems, such as memory leaks, as well as memory management simply having too much of a performance impact on the program. Dr. Sleat also mentioned some safety issues: making sure that the language doesn't expose too much power to the programmer (see Objective 2.11.4) (i.e. allowing the programmer to call the garbage collector directly.) He also says that the most important thing to him is for the language to be clear about exactly how memory is handled.
  ]
)

#question(
  [
    If you were to design the ideal programming language, which features would you want to include?
  ],
  [
    If you look at all of these different programming languages, each of them have exciting bits, and bits that you wouldn't have. I actually used to quite enjoy doing memory allocation myself. It only because of that thrill of the chase to try and find out where memory leaks are. Debugging code is actually quite good fun, and it's a skill that's taken away from you with things like Python, probably rightly so, but that's just who I am. But no, I don't think there's anything particular I would want to see in it.
  ],
  [
    I didn't learn much from the answer of this question, apart from the fact that Dr. Sleat enjoys hunting down memory leaks by hand.
  ]
)

=== Key components
After completing my initial research, these are the key components for the project:

- *Interpreter.* The main goal for my project is to build an interpreter which executes my custom programming language. It will read in the source code, lexically analyse it, parse it, then execute it.

- *Memory management.* Another key component is memory management for data stored on the heap. The interpreter will support naive allocation, reference counting, and mark-and-sweep garbage collection.

- *Language features and syntax.* The language will be imperative, and the syntax will be similar to that of C's. It will be dynamically typed, but also strongly typed to prevent poorly defined operations.

- *Benchmarking.* Various statistics about the performance of the memory management techniques will be measured, including execution time, memory usage, and the number of objects and stack frames at any one moment.

- *Command-line interface.* The interpreter will provide a command-line interface that supports two modes: REPL for interactively executing one line of code at a time, and a file mode for executing full programs.

== Further research

=== Prototype
For the prototype, I decided to implement a simple language, which is only able to handle arithmetic and logical expressions. I chose this prototype because it allows me to understand the implementation of almost all parts of the interpreter (lexer, parser, expression evaluator), without having to go too in-depth into any one part.

#figure(image("../images/p1.png", width: 60%), caption: "Some arithmetic expressions with support for correct operator precedence.")

The prototype is effectively just a calculator: it's unable to handle variable assignments, control flow (other than ternary expressions) or any data structures such as strings. The programmer interacts with the interpreter with a command-line application, using a REPL-like interface, where they input an expression, and the interpreter evaluates and outputs the value of that expression immediately. Any errors which occur throughout the process of lexing, parsing or evaluation are properly caught and reported to the programmer, without crashing the application. While the language is dynamically typed, it is also strongly typed, meaning that poorly defined operations (such as adding an integer to a Boolean), will not be performed.

To assist with creating the prototype, I read chapters 1-6 of _Crafting Interpreters_ @crafting. This book walks through the process of creating an interpreter from the ground up, without using any 'compiler-compilers' or tools which automatically produce a lexer/parser from a language description. The book first implements a tree-walk interpreter using Java, and then implements a bytecode interpreter in C. I used the section which implements the tree-walk interpreter to inform the design of my prototype. This primarily helped me to understand how the lexing and parsing processes which I learned about in _Compilers: Principles, Techniques and Tools_ @dragon are translated into code. While my algorithms are very similar to the ones described in _Crafting Interpreters_, the data structures which I use to represent tokens and expressions are different to those in the book: I made sure to take full advantage of Rust's type system, so that invalid states are not possible to represent. This ensures that weird runtime errors don't crop up.

The code for the prototype is given in @prototype_code, and is also available in the repository for this project @repository.

==== The lexer
The lexer (@prototype_code:lexer) reads in the raw source code from the programmer, and converts it into a stream of `Token`s.

The lexer relies on a wrapper around the source code, called `Source` (@prototype_code:source). This helps to simplify the lexer code, by exposing useful helper methods, such as `matches`, which only advances the stream if the next character matches a specific target character.

As the lexer reads through the source code, it bundles together information into units called `Token`s (@prototype_code:token). A `Token` is comprised of some data, and a location. The data field is where the bulk of the useful information is stored: information about the type of token (e.g. a left parenthesis token), as well as any data the token needs to store (e.g. a number if the token is an integer token) is kept here. The location is simply for debugging purposes later on.

The way in which the lexer works is described in more detail in @analysis:initial:how:lex. It comes down to scanning through the source code, character by character, and deciding which type of token is currently being described, ideally only having to use one or two characters of lookahead to make the judgement.

Occasionally, the programmer may input invalid characters. In this case, the lexer reports a useful error. For example, if the programmer attempts to use a single equals sign--which is not valid (assignment is not implemented in the prototype, whereas equality checks using the double equals are)--then the programmer will be informed that only a single equals character was found, whereas another equals character was expected, as shown in @analysis:prototype:lexer:errors. Other errors, such as characters which are not supported, or keywords which have not been defined (the prototype doesn't support identifiers) will be reported in a similar way.

#figure(image("../images/p2.png", width: 80%), caption: "Error handling at the lexer level.") <analysis:prototype:lexer:errors>

For the (lexically valid) expression `1 + 2 * 3 - 4`, the following tokens are produced:

#block(breakable: false)[
```
Token { data: Integer(1), location: Location { index: 0, line: 1, column: 1 } }
Token { data: Plus,       location: Location { index: 2, line: 1, column: 3 } }
Token { data: Integer(2), location: Location { index: 4, line: 1, column: 5 } }
Token { data: Star,       location: Location { index: 6, line: 1, column: 7 } }
Token { data: Integer(3), location: Location { index: 8, line: 1, column: 9 } }
Token { data: Minus,      location: Location { index: 10, line: 1, column: 11 } }
Token { data: Integer(4), location: Location { index: 12, line: 1, column: 13 } }
```]

We see that for integers, the value of that integer is stored in the `data` field of the `Token`, and that no information from the source code has been lost in the transformation to this new format.

#heading("What I learned", numbering: none, depth: 5)
It was really useful learning how exactly a simple lexer should be implemented in code. There were two important lessons which I learnt from making the prototype lexer. The first one is the importance of having an abstraction over the source code (Objective 2.2). I found that interacting directly with the source code string could be quite annoying, and could introduce some weird bugs if the state of the source code was not all synced up in one place. The helper methods which I built to assist with lexing proved really useful, and made for much more readable code. The second thing I learned was how to best represent tokens and their data. After playing around with multiple options, I settled on using an ```rust enum``` for the token data, with a variant for each type of token. This results in a data structure which can be easily worked with by the parser (Objective 2.7).

==== The parser
The parser (@prototype_code:parser) takes the token stream produced by the lexer, and outputs an abstract syntax tree.

Similarly to the lexer, the parser uses a wrapper over its input, named `TokenStream` (@prototype_code:token_stream). It's packaged with similar helper methods to simplify the code, and make consuming specific tokens easier.

The parser aims to produce an `Expression` tree. An expression can take multiple forms, for example a binary expression, a unary expression or a literal expression. An expression may be composed of sub-expressions, with the 'base' expression being just a literal value (an integer, float or Boolean). The parser produces the expression tree by recursively applying the rules of the grammar, from the top down, making it a  recursive descent parser. The grammar which I developed for the parser is as follows:

#block(breakable: false)[
```
expression -> ternary

ternary -> logical ("?" logical ":" logical)?

logical -> equality (("&&" | "||") equality)*

equality -> comparison (("!=" | "==") comparison)*

comparison -> bitwise ((">" | ">=" | "<" | "<=") bitwise)*

bitwise -> term (("&" | "|") term)*

term -> factor (("+" | "-") factor)*

factor -> unary (("*" | "/") unary)*

unary -> ("!" | "-")? primary

primary -> "(" expression ")"
    | INTEGER
    | FLOAT
    | "true" | "false"
```]

The grammar fully encapsulates the associativity and relative precedence of all of the operators, with ternary operators having the weakest binding, and parenthesis having the strongest binding. This grammar also eliminates _left-recursion_, which would have made writing the parser more difficult if not eliminated.

When a set of syntactically invalid tokens are given to the parser, a detailed error is reported to the programmer. These error handling methods handle situations such as undefined unary operators, missing operands to expressions, or unclosed parenthesis.

#figure(image("../images/p3.png"), caption: "Error reporting at the parser level.")

#block(breakable: false)[
Continuing with the example given in the previous section, the expression tree for `1 + 2 * 3 - 4` is as follows:
```
Binary {
    left: Binary {
        left: Literal(Integer(1)),
        operator: Add,
        right: Binary {
            left: Literal(Integer(2)),
            operator: Multiply,
            right: Literal(Integer(3)),
        },
    },
    operator: Subtract,
    right: Literal(Integer(4)),
}
```]

We see that the expression tree structure shown in @expression_tree is mirrored in the output of the parser, with correct precedence preserved.

#heading("What I learned", numbering: none, depth: 5)
Of all the algorithms which make up the interpreter, the parser was the one which I initially assumed would be the most difficult to implement, so it was reassuring to see that recursive descent is not actually particularly complex. The main lesson which I will take forward into the implementation of the final interpreter is how to handle errors. While I only implemented fairly simple error handling in the prototype, _Crafting Interpreters_ mentioned some more comprehensive error handling methods: all errors should reported, not just the first error which is found. Furthermore, any errors which are simply the result of a previous error (cascaded errors) should not be reported. Also, the importance of maintaining an abstracted layer over the list of tokens became obvious (Objective 2.4). The helper methods I developed were crucial for implementing the parser, as they made the code much more readable and easier to write.

==== The evaluator
The evaluator (@prototype_code:evaluator) traverses the abstract syntax tree produced by the parser, and evaluates it.

The data structure on which the evaluator operates is an expression tree. As described in the previous section, expressions are often composed of sub-expressions. This data structure lends itself to being recursively evaluated.

First, the top--or root--node is visited. Every node visit follows the following process: if the node has any children, then those children are first visited and evaluated. Once the children have been evaluated, the operation which the node represents will be performed on the values of its children. This value is then returned. For literal values (e.g. integers, Booleans etc.), the inner value is simply returned.

Since I decided to make my prototype strongly typed, operations are at this point type checked to ensure that poorly defined operations are not carried out. This means that for every operation, code must be written to handle every possible valid set of operand types. This leads to very verbose code, but since the host language (Rust) is statically typed there doesn't seem to be a simpler way. If the types of the operands to an operation are invalid (e.g. subtracting a float from an integer) then a type error is returned and propagated up the evaluation stack. Ultimately, this will result in no value being returned to the programmer. Another type of error which can occur at this stage is a division by zero: an appropriate error is reported in this case.

#figure(image("../images/p4.png"), caption: "Error reporting at the evaluator level.")

If no errors occur, then the result of evaluating the expression tree is output to the programmer along with the data type of the value.

#figure(image("../images/p5.png", width: 70%), caption: "The output from the evalutor.")

#heading("What I learned", numbering: none, depth: 5)
The first thing I learned is that adding more operations and data types makes for significantly more work in the evaluation stage. I will keep my operations (Objective 1.5) and data types (Objective 1.6) to the bare minimum when creating my final program.

=== Second interview
At this point, I sent Dr. Sleat my prototype as well as my draft objectives to gather feedback. I asked the following questions:

- What features do you like about my prototype?
- Are errors clear and easy to understand?
- Within the current set of features, what do you dislike, and what could be improved (this could be to do with error handling, how type checking is performed etc.)?

He responded with the following:

#quote(block: true, quotes: true)[
Sam

This is beginning to look very interesting indeed. I've had a play around with your prototype and can see that it reports correct answers for similarly typed pairs of numbers. I've also had a go at testing some of the Boolean expressions and this looks good too. What intrigued me is the decision to only allow operators between identically typed variables. This comes out in Objective 1.5.1. I'm guessing that from a memory management perspective it makes it a little simpler not to allow 4 + 9.8 for example?

Your objectives look reasonable. My concern would be that there is a lot here to get done. While I have no doubt in your abilities, do you have a plan for which objectives take priority if you get short on time? Have you planned out subsequent prototypes and defined which objectives will be met in each? This might help you to still achieve your overall objectives with a partially implemented language.

By all means keep using me to test/critique… I'm guessing that when you have a bigger prototype, I might be able to give more concrete feedback.

Kind regards

Phil
]

Dr. Sleat pointed to my decision to enforce very strict type checking for operations. I agree that this can be quite cumbersome, syntax-wise, however I would prefer that my language be very explicit about the types which will arise from operations, even if this comes at the cost of a slightly more verbose syntax. Despite this, I will need ways to cast between values of different types, and some string formatting methods (Objective 1.11)

He also raised the concern about how much there is to get done. I hear these concerns, and I have a plan in place for which objectives will be completed, at what points in time.

I also made my prototype available to other students in my computer science class. It was suggested that I should add support for more operators, such as the right-associative exponent operator. This will be quite interesting to implement, programming language design--wise, so I will implement this within the final interpreter (see Objective 1.5.1.5).

== Objectives
In my project, I will create an interpreter for a custom programming language, and then test various memory management options to see which is the most efficient.

+ There will be a language which the interpreter implements.
  + The language will be imperative.
  + The language will have syntax similar to that of C.
    + Blocks, including subroutine bodies, the bodies of `if`-statements and other control flow structures, will be enclosed in braces.
    + Statements will be terminated by a semicolon.
    + Parameter lists to subroutines will be enclosed in parenthesis.
    + Arithmetic and logical operations will be denoted in infix notation.
    + Line comments will begin with a double slash.
    + Block comments enclosed with `/*` and `*/` will be supported.
  + The language will be dynamically typed.
    + The types of variables and the return types of subroutines do not need to be specified by the programmer.
    + Variables will be able to change type at runtime.
    + Subroutines may return different data types (or no value at all) across different calls to the function.
  + The language will be strongly typed: operations will be type checked to ensure that operations are not performed on incompatible data types.
  + There will be operators.
    + There will be arithmetic infix operators, which may only be performed on a pair of integers, or a pair of floats.
      + The `+` operator adds the right operand to the left operand. This may also be performed on a pair of strings.
      + The `-` operator subtracts the right operand from the left operand.
      + The `*` operator multiplies the left operand by the right operand.
      + The `/` operator divides the left operand by the right operand.
      + The `^` raises the left operand to the power of the right operand.
    + The prefix `-` operator arithmetically negates its operand, and may only be performed on an integer or a float.
    + There will be ordering operators, which may only be performed on a pair of integers, a pair of floats, a pair of strings.
      + The `>` operator checks whether the left operand is greater than the right operand.
      + The `>=` operator checks whether the left operand is greater than or equal to the right operand.
      + The `<` operator checks whether the left operand is less than the right operand.
      + The `<=` operator checks whether the left operand is less than or equal to the right operand.
    + There will be equality operators, which may be performed on any pair of values of the same type.
      + The `==` operator checks whether the left operand is equal to the right operand.
      + The `!=` operator checks whether the left operand is not equal to the right operand.
    + There will be logical operators, which may only be performed on two Boolean values.
      + The `&&` operator checks whether both the left operand and the right operand evaluate to the Boolean true value.
      + The `||` operator checks whether at least one of the left operand or the right operand evaluates to the Boolean true value.
    + There will be bitwise operators, which may only be performed on two Boolean values, or two integers.
       + The `&` operator performs a bitwise AND with the left operand and the right operand.
       + The `|` operator performs a bitwise OR with the left operand and the right operand.
    + There will be the prefix `!` operator, which is the logical negation operator for a Boolean operand, and the bitwise negation operator for an integer operand.
    + There will be the ternary `<condition> ? <if_true> : <if_false>` operator, which evaluates to `<if_true>` if `<condition>` is true, and `<if_false>` otherwise.
    + The language will respect operator precedence as described in @grammar.
  + There will be built-in data types.
    + There will be the integer type, which stores a 32-bit signed integer.
    + There will be the float type, which stores a 64-bit signed floating point number.
    + There will be the Boolean type, which stores a Boolean value.
    + There will be an string type.
  + There will be user-defined objects, which are composed of data items.
    + User-defined objects will be stored on the heap.
    + User-defined objects will store a number of data items, each of which has an identifier.
    + Data items will be accessible via the combination of an object name and item identifier.
    + User-defined objects will have no templates, or classes.
    + User-defined objects will have no methods defined on them.
    + User-defined objects will have no access restrictions on their data items.
  + There will be user-definable variables.
    + Variables will be stored on the stack.
    + Variables will have an identifier.
    + Variables will either store a primitive value, or an object reference.
    + Variables will only exist within the scope they have been defined in.
      + There will be local variables, which are defined within subroutines.
      + There will be global variables, which are defined at the outer layer of the program.
    + Variables will be mutable.
  + There will be control flow constructs.
    + There will be `if` statements.
      + An `if` statement evaluates a condition, and executes the following block if the condition evaluates to the Boolean true value.
    + There will be `else` statements.
      + An `else` statement must appear after an `if` statement.
      + The following block will be executed if previous `if` statement conditions have not passed.
    + There will be `while` loops.
      + A `while` loop evaluates a condition, then executes the following block if the condition evaluates to the Boolean true value.
      + This will happen repeatedly, until the condition evaluates to the Boolean false value.
  + There will be user-definable subroutines.
    + Subroutines will have an identifier.
    + Subroutines will have a list of parameters.
      + Parameters will not have set data types.
      + Subroutines must be called with the number of arguments equal to the number of parameters.
    + Subroutines will have a body, which is the list of statements that should be executed when the function is called.
    + Subroutines will be able to be called.
    + Subroutines will be able to call subroutines, including themselves (recursion).
    + Subroutines may return a value, with no set data type.
  + There will be native subroutines, which are provided by the language.
    + There will be a `print` function, which prints to standard output.
    + There will be a `format` function, which allows injecting values of various types into strings.
    + There will be an `input` function, which receives input from standard input.
    + There will be an `int` function, which attempts to cast the input to an integer.
    + There will be a `float` function, which attempts to cast the input to a float.
+ There will be an interpreter, which implements the language.
  + The interpreter will execute the program from source.
  + The lexer will not interact with the source code directly, instead wrapping it in an abstraction.
  + The lexer will only require two characters of lookahead.
  + The parser will not interact with the token list directly, instead wrapping it in an abstraction.
  + The parser will use recursive descent parsing.
  + Expressions will be evaluated using a post-order traversal of the AST.
  + Tokens will comprised of token data (which will be stored using an ```rust enum```), and the token's location.
  + Any errors which occur throughout the interpretation process will be reported to the programmer.
    + If the error occurred while lexing or parsing, the line and column at which the error occurred will be displayed to the programmer.
    + The cause of the error will be displayed to the programmer.
  + There will be a stack, which stores a collection of stack frames.
    + Each time a subroutine is called, a new stack frame will be pushed onto the stack.
    + Each time a subroutine returns, its stack frame will be popped off of the stack.
    + Each stack frame will store a collection of local variables.
    + Each stack frame will store any arguments which were passed to it.
  + There will be a heap, which stores a collection of unorganised data.
    + Only objects may be stored on the heap.
    + Data on the heap will not automatically cleaned up when no longer needed (unless using a memory management technique).
  + There will be three options for managing memory stored on the heap.
    + One option will be naive allocation, where no memory is ever freed.
    + One option will be reference counting.
      + Each value stored in the heap will store how many references there are to it.
      + When an object references another object, the reference count of the object it references will be incremented.
      + When an object is deleted, the reference counts of any objects it references will be decremented.
      + When the reference count for a value drops to zero, the value will be deleted.
    + One option will be mark and sweep garbage collection.
      + The garbage collector will 'mark' every object in the heap which is reachable from the stack.
      + Every object which is unmarked will be deleted.
      + The garbage collector will be called at regular intervals.
    + The programmer will not be able to invoke the memory management technique themself.
+ The performance of the memory management techniques will be benchmarked.
  + A variety of algorithms will be implemented in the algorithm.
    + The Fibonacci function will be implemented, in order to test out recursion.
    + A program which introduces reference cycles between objects will be implemented.
  + Statistics about the execution of the program will be tracked.
    + The execution time of the program.
    + The number of objects at any one moment.
    + The number of stack frames at any one moment.
    + The memory usage of the interpreter at any one moment.
+ The programmer will interact with the interpreter through a command-line application.
  + The programmer will pass in their desired memory management mode as an argument.
  + The command-line application will have a 'REPL' mode.
    + The programmer will then be able to input one line of source code at a time.
    + This line will then be executed by the interpreter.
    + The environment (variables, function definitions etc.) will be maintained between each line.
    + Any errors will be reported to the programmer, and will not crash the command-line application.
  + The command-line application will have a file mode.
    + The programmer will pass in a file path as an argument to the command-line application.
    + The application will read the contents of the file, which will then be executed by the interpreter.
    + If an error occurs, then interpretation of the file will end.
    + Statistics about the execution of the program will be written to a CSV file.
  + If the programmer passes a file path as an argument to the command-line application, then the file mode will be enabled. Otherwise, the REPL mode will be enabled.

== Modelling

=== Grammar <grammar>
The grammar for my programming language is as follows. It is written in a form similar to BNF, like `non-terminal -> productions`, and uses EBNF operators.

```
program -> statement*

statement -> variableDeclaration
           | functionDefinition
           | returnStatement
           | ifStatement
           | whileLoop
           | block
           | expressionStatement

expressionStatement -> expression ";"

variableDeclaration -> "let" IDENTIFIER ("=" expression)? ";"

functionDefinition -> "fu" IDENTIFIER "(" (IDENTIFIER ("," IDENTIFIER)*)? ")" block

returnStatement -> "return" expression? ";"

ifStatement -> "if" expression block ("else" (block | ifStatement))?

whileLoop -> "while" expression block

block -> "{" statement* "}"

expression -> assignment

assignment -> (call ".")? IDENTIFIER "=" assignment
            | ternary

ternary -> logical ("?" logical ":" logical)?

logical -> equality (("&&" | "||") equality)*

equality -> comparison (("!=" | "==") comparison)*

comparison -> bitwise ((">" | ">=" | "<" | "<=") bitwise)*

bitwise -> term (("&" | "|") term)*

term -> factor (("+" | "-") factor)*

factor -> unary (("*" | "/") unary)*

unary -> ("!" | "-")? exponent

exponent -> call ("^" exponent)?

call -> primary ( ("(" (expression ("," expression)*)? ")") | ("." IDENTIFIER) )*

primary -> "(" expression ")"
         | STRING
         | INTEGER
         | FLOAT
         | IDENTIFIER
         | object
         | "true" | "false"

object -> "{" (IDENTIFIER ":" expression ("," IDENTIFIER ":" expression)*)? "}"
```

It defines the various forms of statements and expressions that are valid within the language, and encodes operator precedence and associativity. `assignment` expressions have the least precedence, whereas `primary` expressions have the most precedence. The grammar ignores whitespace, because it can all be safely ignored. I plan on using this grammar to write my parser, as each non-terminal within the grammar can typically correspond to one subroutine in the parser source code (within a recursive-descent parser).

=== Flowchart
The flowchart below describes the high-level operation of the interpreter. The user will be able to specify a memory management technique (either garbage collected, reference counted, or naive) which will be used to manage the heap for the execution of their program. If the user decided to run an entire file, then statistics about the performance of the program will be displayed after execution finishes.

#image("../images/High-level operation.svg")