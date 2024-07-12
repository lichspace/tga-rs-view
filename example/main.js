import init, { start, read_tga, read_rgb } from "../pkg/tgaviewer.js"

async function main() {
  await init();
  start();

  console.time('tga decode');
  const tga = await fetch('./30.tga');
  const buffer = await tga.arrayBuffer();
  const u8 = new Uint8Array(buffer);

  const canvas = document.querySelector('#canvas');

  // canvas.width = imagedata.width;
  // canvas.height = imagedata.height;
  const ctx = canvas.getContext('2d');
  read_tga(ctx, u8);
  console.timeEnd('tga decode');
  console.log('分辨率', canvas.width, canvas.height, canvas.width * canvas.height * 3)

  console.time('read_rgb');
  const rgb = read_rgb(ctx, u8);
  console.timeEnd('read_rgb');

  console.log(rgb);
}

main()