import rust, { Canvas } from '../crate/Cargo.toml' 

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

const stepBtn = document.getElementById('step-btn')
stepBtn.addEventListener('click', step)

let viewColors = true
const intensityBtn = document.getElementById('intensity')
intensityBtn.addEventListener('click', () => { viewColors = !viewColors })

const FPSCounter = document.getElementById('fps')
const frameCounter = document.getElementById('frameCount')

const norm = document.getElementById('norm');


function step () {
    rustCanvas.step(0, !viewColors);
    frameCount++
    frameCounter.innerText = frameCount
    norm.innerText = rustCanvas.norm();
    const imagePtr = rustCanvas.image();
    const imageArray = new Uint8ClampedArray(rust.wasm.memory.buffer, imagePtr, 4 * height * width);
    let imageData = new ImageData(imageArray, width, height);
    ctx.putImageData(imageData, 0, 0);
}

let t0 = 0;
let t1 = 1;
let duration = t0 - t1;
let frameCount = 0
let id = null;


function render(){
    id = null;
    t0 = performance.now()
    step()
    t1 = performance.now()
    duration = t1 - t0
    run();
};

function run (){
    if(frameCount % 10 === 0) FPSCounter.innerText = Math.floor(1e3/duration)
    if (!id) { id = requestAnimationFrame(render) }
}

let start = document.getElementById('start');
let running = false;
start.addEventListener('click', () => {
    running = !running;
    if(running) { 
        run(); 
        start.innerHTML = 'Stop';
        start.style.backgroundColor = 'red';
    }
    else { 
        cancelAnimationFrame(id); 
        id = null; 
        start.innerHTML = 'Start';
        start.style.backgroundColor = 'greenyellow';
    }
});
