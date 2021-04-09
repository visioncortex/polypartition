export function clearCanvas(canvas: HTMLCanvasElement) {
    let ctx = canvas.getContext('2d');
    ctx.fillStyle = "#FFFFFF";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}