import { Tester } from "polypartition";
import { readFile } from "./fileio";

async function main() {
    const inputText = await readFile("../assets/test_input.txt");
    const tester = Tester.from_input_text(inputText);
    tester.draw_polygons('main', 'in');
}

main().catch(console.error);