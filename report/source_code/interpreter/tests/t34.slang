fu layer1() {
    print(format("start of layer1 - stack frames count: ", STACK_FRAMES_COUNT));
    layer2();
    print(format("end of layer1 - stack frames count: ", STACK_FRAMES_COUNT));
}

fu layer2() {
    print(format("start of layer2 - stack frames count: ", STACK_FRAMES_COUNT));
    layer3();
    print(format("end of layer2 - stack frames count: ", STACK_FRAMES_COUNT));
}

fu layer3() {
    print(format("layer3 - stack frames count: ", STACK_FRAMES_COUNT));
}

print(format("start of outer layer - stack frames count: ", STACK_FRAMES_COUNT));
layer1();
print(format("end of outer layer - stack frames count: ", STACK_FRAMES_COUNT));