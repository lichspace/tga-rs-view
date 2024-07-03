import init, { start, read_tga } from "../pkg/tgaviewer.js"

async function main() {
  await init();
  start();

  console.time('tga decode');
  const tga = await fetch('./yexiu.tga');
  const buffer = await tga.arrayBuffer();
  const u8 = new Uint8Array(buffer);
  console.log(u8.length);

  const canvas = document.querySelector('#canvas');
  // canvas.width = imagedata.width;
  // canvas.height = imagedata.height;
  const ctx = canvas.getContext('2d');
  read_tga(ctx, u8);
  console.timeEnd('tga decode');

  console.log('分辨率', canvas.width, canvas.height)
}

main()