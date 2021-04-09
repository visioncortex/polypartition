import { Tester } from "polypartition";
import { readFile } from "./fileio";

const IN = 'in';
const OUT = 'out';
const BOTH = 'both';

async function main() {
    const inputText = await readFile("../assets/test_input.txt");
    const tester = Tester.from_input_text(inputText);
    tester.draw_polygons('main', IN);
    
    run_test_remove_holes(tester, 'main');
}

main().catch(console.error);

function run_test_remove_holes(tester: Tester, canvasId: string, verbose?: string) {
    try {
        tester.test_remove_holes();
        tester.draw_polygons(canvasId, OUT);
        if ([IN, OUT].includes(verbose)) {
            tester.print(verbose);
        } else if (verbose === BOTH) {
            tester.print(IN);
            tester.print(OUT);
        }
    } catch (e) {
        console.error(e);
    }
}