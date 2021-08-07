import rust, { Canvas } from '../crate/Cargo.toml' 

let canvas = document.getElementById('canvas');
const rustCanvas = Canvas.new(canvas.width, canvas.height);
const width = rustCanvas.width();
const height = rustCanvas.height();
let ctx = canvas.getContext('2d');

const stepBtn = document.getElementById('step-btn')
stepBtn.addEventListener('click', step)

let viewColors = true
const intensityBtn = document.getElementById('intensity')
intensityBtn.addEventListener('click', () => { viewColors = !viewColors })

// Debug tools --------------------------------------------

const FPSCounter = document.getElementById('fps')
const frameCounter = document.getElementById('frameCount')

const norm = document.getElementById('norm');

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


const hoveredColor = document.getElementById('hoveredColor');

function pick(event, destination) {
    const x = event.layerX - event.target.offsetLeft;
    const y = event.layerY - event.target.offsetTop;
    const pixel = ctx.getImageData(x, y, 1, 1);
    const data = pixel.data;
    const [r,g,b,a] = data
    // console.log(event)

    const rgba_to_hue = (args) => {
        const normalize = v => v / 255
        const normalized = Array.from(args).map(v => normalize(v))
        const xMax = Math.max(...normalized)
        const xMin = Math.min(...normalized)
        const [r,g,b] = normalized
        const chroma = xMax - xMin
        if (chroma === 0) return 0
        if (xMax === r) return 60*(g-b)/chroma
        if (xMax === g) return 60*(2 + (b-r)/chroma)
        if (xMax === b) return 60*(4 + (r-g)/chroma)
    }

    const rgba = `rgba(${r}, ${g}, ${b}, ${a / 255})`;
    destination.style.background = rgba;
    destination.textContent = `${rgba_to_hue(data.slice(0,3)).toFixed(0)} degrees`;

    return rgba;
}

canvas.addEventListener('mousemove', function(evt) {
    pick(evt, hoveredColor)
})

// ---------------------------------------------------------------

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
