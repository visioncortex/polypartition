import { DrawingUtil } from "polypartition";

const mainCanvas = document.getElementById('main') as HTMLCanvasElement;
const mainCtx = mainCanvas.getContext('2d');

mainCtx.fillStyle = "#FFFFFF";
mainCtx.fillRect(0, 0, mainCanvas.width, mainCanvas.height);

const drawingUtil = DrawingUtil.from_canvas_id('main');
drawingUtil.draw_line(5, 4, 50, 100);