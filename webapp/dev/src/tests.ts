import { Tester } from "polypartition";
import { readFile } from "./fileio";

export const IN = 'in';
export const OUT = 'out';
export const BOTH = 'both';

export default [
    async function runTestRemoveHoles(canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile("../assets/test_input.txt");
            tester = Tester.from_input_text(inputText);
            tester.test_remove_holes();
            tester.draw_polygons(canvasId, OUT);
            if ([IN, OUT].includes(verbose)) {
                tester.print(verbose);
            } else if (verbose === BOTH) {
                tester.print(IN);
                tester.print(OUT);
            }
        } catch (e) {
            throw e;
        } finally {
            if (tester) {
                tester.free();
            }
        }
    },
    async function runTestEarClipping(canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile("../assets/test_input.txt");
            tester = Tester.from_input_text(inputText);
            tester.test_ear_clipping();
            tester.draw_polygons(canvasId, OUT);
            if ([IN, OUT].includes(verbose)) {
                tester.print(verbose);
            } else if (verbose === BOTH) {
                tester.print(IN);
                tester.print(OUT);
            }
        } catch (e) {
            throw e;
        } finally {
            if (tester) {
                tester.free();
            }
        }
    },
];