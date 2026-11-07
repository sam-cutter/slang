// Must be run in reference counting ("rc") mode.

print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

{
    print("Scope entered.");
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));
    let x = {};
    print("Object x created.");
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));
}

print("Scope exited.");
print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

fu create_object() {
    print("New stack frame entered");
    let y = {};
    print("Object y created.");
    print(format("Current objects count: ", HEAP_OBJECTS_COUNT));
}

create_object();

print("Stack frame exited.");
print(format("Current objects count: ", HEAP_OBJECTS_COUNT));