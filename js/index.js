import { memory, Arena } from '../crate/Cargo.toml' // This might be incorrect

const arena = Arena.new();
const width = arena.width();
const height = arena.height();

let canvas = document.getElementById('canvas');
let ctx = canvas.getContext('2d');

const drawSound = () => {
    const imagePtr = arena.image();
    const imageArray = new Uint8ClampedArray(memory.buffer, imagePtr, 4 * height * width);
    let imageData = new ImageData(imageArray, width, height);
    ctx.putImageData(imageData, 0, 0);
};

const forcePtr = arena.force();
const forceArray = new Int32Array(memory.buffer, forcePtr, height * width);

const windowToCanvas = (canvas, x, y) => {
    const bbox = canvas.getBoundingClientRect();
    return {
        x: Math.round(x - bbox.left * (canvas.width / bbox.width)),
        y: Math.round(y - bbox.top * (canvas.height / bbox.height))
    };
};

canvas.onmousedown = e => {
    e.preventDefault();
    const loc = windowToCanvas(canvas, e.clientX, e.clientY);
    const targetIndex = loc.x + loc.y * width;
    forceArray[targetIndex] = 0x3fffffff;
};

let running = false;
const render = ({loop = true, debug = false}) => {
    arena.step(0);
    drawSound();
    if(loop){
        if(running) requestAnimationFrame(render);
    }
};

let debug = document.getElementById('debug').checked;

let intensity = document.getElementById('view').checked;

let step = document.getElementById('step');
step.addEventListener('click', () => {
  render({debug});  
});

let start = document.getElementById('start');
start.addEventListener('click', () => {
    running = !running;
    if(running) start.innerHTML = 'Stop';
    else start.innerHTML = 'Start';
    render({loop: true, debug});
});
