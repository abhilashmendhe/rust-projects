struct WiredKeyboard {
    name: String
}
struct WiredMouse {
    name: String
}

struct MacBook {
    wire_keyboard: WiredKeyboard,
    wire_mouse: WiredMouse,
}

// Bad
/*
    We could also have a wireless keyboard or a wireless mouse
    Therefore, create a trait/interface keyboard that implements both wired and wireless keyboard.
    Do same for mouse.
*/