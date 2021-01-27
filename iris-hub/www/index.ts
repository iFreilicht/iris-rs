import * as wasm from "iris-hub";


// The ring display SVG object
const ring_display: Document = (document.querySelector("#ring-display") as HTMLObjectElement).contentDocument!

function update_display() {
    for (let channel = 0; channel < 12; channel++) {
        let color = wasm.current_color(Date.now() % 4294967295, channel)
        let svg_element = ring_display.getElementById("channel" + channel)
        if (svg_element){
            svg_element.setAttribute("stroke", color)
        } else {
            console.warn("Channel " + channel + " does not exist!")
        }
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
