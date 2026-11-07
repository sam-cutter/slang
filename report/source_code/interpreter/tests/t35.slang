// Must be run in naive allocation ("na") mode.

print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

let x = {};
print("Object x created.");

print(format("Current objects count: ", HEAP_OBJECTS_COUNT));

let y = 10;
print("Integer x created.");

print(format("Current objects count: ", HEAP_OBJECTS_COUNT));