import { Laboratory } from '../crate/Cargo.toml'; 

const lab = Laboratory.new();
const width = lab.width();
const height = lab.height();

let canvas = document.getElementById('canvas');
let ctx = canvas.getContext('2d');

const drawPsi = () => {
    const imagePtr = lab.image();
    const imageArray = new Uint8ClampedArray(memory.buffer, imagePtr, 4 * height * width);
    let imageData = new ImageData(imageArray, width, height);
    ctx.putImageData(imageData, 0, 0);
};

let running = false;
const render = ({loop = true, debug = false}) => {
    lab.step();
    drawPsi();
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
    start.innerHTML = 'Stop';
    running = !running;
    render({loop: true, debug});
});
