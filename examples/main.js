import init, { Rimage } from "../pkg/tgaviewer.js"

async function main() {
  await init();
  // start();

  const tga = await fetch('./30.tga');
  const buffer = await tga.arrayBuffer();
  const u8 = new Uint8Array(buffer);

  const cl = new Rimage(0);
  cl.open_tga(u8);
  const [width, height] = cl.size;

  const data = cl.data;

  console.log(width, height, data);

  const canvas = document.querySelector('#canvas');
  canvas.width = width;
  canvas.height = height;
  const ctx = canvas.getContext('2d');
  ctx.putImageData(new ImageData(new Uint8ClampedArray(data), width, height), 0, 0);

  canvas.addEventListener('pointerdown', e => {
    const { offsetX, offsetY } = e;
    console.time('flood_fill');
    const data = cl.flood_fill(offsetX, offsetY, new Uint8Array([255, 0, 0, 255]));
    const rect = cl.flood_fill_rect;
    console.timeEnd('flood_fill');
    const canvas = document.createElement('canvas');
    canvas.width = width;
    canvas.height = height;
    const ctx2 = canvas.getContext('2d');
    console.log(data, rect);
    ctx2.putImageData(new ImageData(new Uint8ClampedArray(data), rect[2], rect[3]), rect[0], rect[1]);
    ctx.drawImage(canvas, 0, 0);
  });
}

main()