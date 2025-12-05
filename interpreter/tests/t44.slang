// Must be run in garbage collection ("gc") mode.

{
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

    let x = {};
    print("Object x created.");
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

    x = 5;
    print("x reassigned to 5.");

    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

    let y = {};
    print("Object y created.");
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

    let y = 10;
    print("y redefined as 10.");

    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));
}

print(format("Current objects count: ", HEAP_OBJECTS_COUNT));