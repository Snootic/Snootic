import * as THREE from "three";

export function createCard(name, title, available, R, BAR_H) {
  const cardCanvas = document.createElement("canvas");
  const W = 340;
  const H = 200;
  const texScale = 1;
  cardCanvas.width = W * texScale;
  cardCanvas.height = H * texScale;
  const ctx = cardCanvas.getContext("2d");
  ctx.scale(texScale, texScale);

  R = 12;
  BAR_H = 36;

  ctx.beginPath();
  ctx.roundRect(0, 0, W, H, R);
  ctx.fillStyle = "#1a1714";
  ctx.fill();

  ctx.strokeStyle = "#3a3530";
  ctx.lineWidth = 1;
  ctx.beginPath();
  ctx.roundRect(0.5, 0.5, W - 1, H - 1, R);
  ctx.stroke();

  ctx.save();
  ctx.beginPath();
  ctx.roundRect(1, 1, W - 2, BAR_H, [R - 1, R - 1, 0, 0]);
  ctx.clip();
  ctx.fillStyle = "#211e1a";
  ctx.fillRect(0, 0, W, BAR_H);
  ctx.restore();

  ctx.strokeStyle = "#3a3530";
  ctx.beginPath();
  ctx.moveTo(0, BAR_H);
  ctx.lineTo(W, BAR_H);
  ctx.stroke();

  const dotY = BAR_H / 2;
  for (const [color, cx] of [
    ["rgba(239,68,68,0.7)", 16],
    ["rgba(234,179,8,0.7)", 28],
    ["rgba(16,185,129,0.7)", 40],
  ]) {
    ctx.fillStyle = color;
    ctx.beginPath();
    ctx.arc(cx, dotY, 4, 0, Math.PI * 2);
    ctx.fill();
  }

  ctx.fillStyle = "#6e6a65";
  ctx.font = "11px monospace";
  ctx.textAlign = "right";
  ctx.fillText("player.rs", W - 16, dotY + 4);

  ctx.textAlign = "left";
  ctx.font = "12px monospace";
  const lh = 18;
  let y = BAR_H + 24;
  const x0 = 16;
  const ind = 36;

  const write = (pairs, startX) => {
    let x = startX;
    for (const [text, color] of pairs) {
      ctx.fillStyle = color;
      ctx.fillText(text, x, y);
      x += ctx.measureText(text).width;
    }
  };

  write(
    [
      ["let ", "#CC7A5A"],
      ["Snootic", "#38bdf8"],
      [" = PlayerTraits {", "#a3a09c"],
    ],
    x0,
  );
  y += lh;
  write(
    [
      ["Name: ", "#a3a09c"],
      [`'${name}'`, "#34d399"],
      [",", "#a3a09c"],
    ],
    ind,
  );
  y += lh;
  write(
    [
      ["Role: ", "#a3a09c"],
      [`'${title}'`, "#34d399"],
      [",", "#a3a09c"],
    ],
    ind,
  );
  y += lh;
  write(
    [
      ["Status: ", "#a3a09c"],
      [`'${available}'`, "#34d399"],
    ],
    ind,
  );
  y += lh;
  write(
    [
      ["}", "#a3a09c"],
      [" \u258C", "#6e6a65"],
    ],
    x0,
  );
  return {cardCanvas, ctx};
}

export function createCardTexture(card) {
  const tex = new THREE.CanvasTexture(card);
  tex.colorSpace = THREE.SRGBColorSpace;
  tex.generateMipmaps = false;
  tex.minFilter = THREE.LinearFilter;
  tex.magFilter = THREE.LinearFilter;
  
  return tex
}

export function updateCardDot(ctx, BAR_H, elapsed) {
  const cursorAlpha = Math.abs(Math.sin(elapsed * 3));
  const bar = BAR_H + 24
  
  ctx.fillStyle = "#1a1714";
  ctx.fillRect(16, bar + 96 - 12, 40, 20);

  ctx.fillStyle = "#a3a09c";
  ctx.fillText("}", 16, bar + 96);
  ctx.fillStyle = `rgba(110, 106, 101, ${cursorAlpha})`;
  ctx.fillText(" \u258C", 16 + 8, bar + 96);
}