import { DrawingUtil } from "polypartition";
var mainCanvas = document.getElementById('main');
var mainCtx = mainCanvas.getContext('2d');
mainCtx.fillStyle = "#FFFFFF";
mainCtx.fillRect(0, 0, mainCanvas.width, mainCanvas.height);
var drawingUtil = DrawingUtil.from_canvas_id('main');
drawingUtil.draw_line(5, 4, 50, 100);
//# sourceMappingURL=index.js.map