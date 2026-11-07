// Must be run in reference counting ("rc") mode.

fu create_object() {
    print(format("(a) Current objects count: ", HEAP_OBJECTS_COUNT));
    let y = {};
    print(format("(b) Current objects count: ", HEAP_OBJECTS_COUNT));

    return y;
}

{
    print(format("(1) Current objects count: ", HEAP_OBJECTS_COUNT));

    create_object();

    print(format("(2) Current objects count: ", HEAP_OBJECTS_COUNT));
}

print(format("(3) Current objects count: ", HEAP_OBJECTS_COUNT));

{
    print(format("(4) Current objects count: ", HEAP_OBJECTS_COUNT));

    let x = create_object();

    print(format("(5) Current objects count: ", HEAP_OBJECTS_COUNT));
}

print(format("(6) Current objects count: ", HEAP_OBJECTS_COUNT));