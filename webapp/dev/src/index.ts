import { Tester } from "polypartition";
import { clearCanvas } from "./canvas";
import { readFile } from "./fileio";

const mainCanvas = document.getElementById('main') as HTMLCanvasElement;

clearCanvas(mainCanvas);

async function main() {
    const inputText = await readFile("../assets/test_input.txt");
    const tester = Tester.from_input_text(inputText);
    tester.draw_polygons('main', 'in');
}

main().catch(console.error);