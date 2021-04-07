import { DrawingUtil } from "polypartition";

const mainCanvas = document.getElementById('main') as HTMLCanvasElement;
const mainCtx = mainCanvas.getContext('2d');

function clearCanvas(canvas: HTMLCanvasElement) {
    let ctx = canvas.getContext('2d');
    ctx.fillStyle = "#FFFFFF";
    ctx.fillRect(0, 0, canvas.width, canvas.height);
}

clearCanvas(mainCanvas);

const drawingUtil = DrawingUtil.from_canvas_id('main');
drawingUtil.draw_line(5, 4, 50, 100);
drawingUtil.draw_line(400, 300, 250, 350);