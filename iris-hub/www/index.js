import * as wasm from "iris-hub";

const ring_display = document.querySelector("#ring-display").contentDocument

function update_display() {
    for (let channel = 0; channel < 12; channel++) {
        let channel_color = wasm.current_color(Date.now() % 4294967295, channel)
        ring_display.getElementById("channel" + channel).setAttribute("stroke", channel_color)
    }
}

// Add a single default cue so there's something to display
wasm.add_cue()
wasm.launch_cue(0)

wasm.init()
// Update display indefinitely
window.setInterval(function () {
    update_display()
}, 50)
