import rust, { Canvas } from '../crate/Cargo.toml' 

console.log(Canvas, rust.wasm.memory)
let canvas = document.getElementById('canvas');
const rustCanvas = Canvas.new(canvas.width, canvas.height);
const width = rustCanvas.width();
const height = rustCanvas.height();
let ctx = canvas.getContext('2d');

const debug = () => {
    let debugCanvas = document.getElementById('test-coloring');
    let debugCtx = debugCanvas.getContext('2d');
    const testPtr = rustCanvas.test();
    const testArray = new Uint8ClampedArray(rust.wasm.memory.buffer, testPtr, 4 * height * width);
    let testData = new ImageData(testArray, width, height);
    debugCtx.putImageData(testData, 0, 0);
}

const debugBtn = document.getElementById('debug-btn')
debugBtn.addEventListener('click', debug)


// let intensity = document.getElementById('view').checked;

// let step = document.getElementById('step');
// step.addEventListener('click', () => {
//   render({debug});  
// });

// canvas.onmousedown = e => {
//     e.preventDefault();
//     const loc = windowToCanvas(canvas, e.clientX, e.clientY);
//     const targetIndex = loc.x + loc.y * width;
//     forceArray[targetIndex] = 0x3fffffff;
// };

// const windowToCanvas = (canvas, x, y) => {
//     const bbox = canvas.getBoundingClientRect();
//     return {
//         x: Math.round(x - bbox.left * (canvas.width / bbox.width)),
//         y: Math.round(y - bbox.top * (canvas.height / bbox.height))
//     };
// };
let t0 = 0;
let t1 = 1;
let duration = t0 - t1;
let frameCount = 0

const FPSCounter = document.getElementById('fps')

let id = null;


function render(){
    id = null;
    t0 = performance.now()
    frameCount++

    rustCanvas.step();
    const imagePtr = rustCanvas.image();
    const imageArray = new Uint8ClampedArray(rust.wasm.memory.buffer, imagePtr, 4 * height * width);
    let imageData = new ImageData(imageArray, width, height);
    ctx.putImageData(imageData, 0, 0);
    t1 = performance.now()
    duration = t1 - t0
    run();
};

function run (){
    if(frameCount %10 === 0) FPSCounter.innerText = Math.floor(1e3/duration)
    if (!id) { id = requestAnimationFrame(render) }
}

let start = document.getElementById('start');
let running = false;
start.addEventListener('click', () => {
    running = !running;
    if(running) { run(); start.innerHTML = 'Stop'; }
    else { cancelAnimationFrame(id); id = null; start.innerHTML = 'Start';  }
});
