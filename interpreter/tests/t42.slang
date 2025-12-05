// Must be run in garbage collection ("gc") mode.

fu create_cycle(x) {
    let y = {other: x};

    print("Object y created.");
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

    x.other = y;

    print("Cycle created.");
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));
}

{
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

    let x = {};
    print("Object x created.");

    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

    create_cycle(x);

    print("Subroutine create_cycle exited.");
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));
}

print("Scope exited.");
print(format("Current objects count: ", HEAP_OBJECTS_COUNT));