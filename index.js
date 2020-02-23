import Laboratory from './src/lib.rs';

const lab = Laboratory.new();
const width = lab.width();
const height = lab.height();

let canvas = document.getElementById('canvas');
let ctx = canvas.getContext('2d');

// const drawPsi = () => {
//     const imagePtr = lab.image();
//     const imageArray = new Uint8ClampedArray(memory.buffer, imagePtr, 4 * height * width);
//     let imageData = new ImageData(imageArray, width, height);
//     ctx.putImageData(imageData, 0, 0);
// }

// const renderLoop = () => {
//     lab.step();
//     drawPsi();
//     requestAnimationFrame(renderLoop)
// }

let btn = document.getElementById('start');
btn.addEventListener('click', () => renderLoop());