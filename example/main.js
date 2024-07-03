import init, { start, read_tga, rgba_to_rgb } from "../pkg/tgaviewer.js"

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
  console.log('分辨率', canvas.width, canvas.height)
  const rgb = rgba_to_rgb(ctx, canvas.width, canvas.height);
  console.log(rgb);
}

main()