import { Tester } from "polypartition";
import { createPathToAsset, readFile } from "./fileio";

export const IN = 'in';
export const OUT = 'out';
export const BOTH = 'both';

export default [
    async function RenderInput(canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile(createPathToAsset("test_input.txt"));
            tester = Tester.from_input_text(inputText);
            tester.draw_polygons(canvasId, IN);
            if ([IN, OUT].includes(verbose)) {
                tester.print(verbose);
            } else if (verbose === BOTH) {
                tester.print(IN);
                tester.print(OUT);
            }
            const dump = tester.dump_polygons(IN, false);
            if (dump.localeCompare(inputText) !== 0) {
                throw `Dump Incorrect!\n\nExpected:\n${inputText}Dump:\n${dump}`;
            }
        } catch (e) {
            throw e;
        } finally {
            if (tester) {
                tester.free();
            }
        }
    },
    async function RemoveHoles(canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile(createPathToAsset("test_input.txt"));
            tester = Tester.from_input_text(inputText);
            tester.test_remove_holes();
            tester.draw_polygons(canvasId, OUT);
            if ([IN, OUT].includes(verbose)) {
                tester.print(verbose);
            } else if (verbose === BOTH) {
                tester.print(IN);
                tester.print(OUT);
            }
            const dump = tester.dump_polygons(OUT, false);
            const outputText = await readFile(createPathToAsset("test_remove_holes.txt"));
            if (dump.localeCompare(outputText) !== 0) {
                throw `Dump Incorrect!\n\nExpected:\n${outputText}Dump:\n${dump}`;
            }
        } catch (e) {
            throw e;
        } finally {
            if (tester) {
                tester.free();
            }
        }
    },
    async function EarClipping(canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile(createPathToAsset("test_input.txt"));
            tester = Tester.from_input_text(inputText);
            tester.test_ear_clipping();
            tester.draw_polygons(canvasId, OUT);
            if ([IN, OUT].includes(verbose)) {
                tester.print(verbose);
            } else if (verbose === BOTH) {
                tester.print(IN);
                tester.print(OUT);
            }
            const dump = tester.dump_polygons(OUT, false);
            const outputText = await readFile(createPathToAsset("test_triangulate_EC.txt"));
            if (dump.localeCompare(outputText) !== 0) {
                throw `Dump Incorrect!\n\nExpected:\n${outputText}Dump:\n${dump}`;
            }
        } catch (e) {
            throw e;
        } finally {
            if (tester) {
                tester.free();
            }
        }
    },
    async function OptimalDP(canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile(createPathToAsset("test_input_no_hole.txt"));
            tester = Tester.from_input_text(inputText);
            tester.test_optimal_dp();
            tester.draw_polygons(canvasId, OUT);
            if ([IN, OUT].includes(verbose)) {
                tester.print(verbose);
            } else if (verbose === BOTH) {
                tester.print(IN);
                tester.print(OUT);
            }
            const dump = tester.dump_polygons(OUT, false);
            const outputText = await readFile(createPathToAsset("test_triangulate_OPT.txt"));
            if (dump.localeCompare(outputText) !== 0) {
                throw `Dump Incorrect!\n\nExpected:\n${outputText}Dump:\n${dump}`;
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