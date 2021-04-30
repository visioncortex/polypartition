import { Tester } from "polypartition";
import { createPathToAsset, readFile } from "./fileio";

export const IN = 'in';
export const OUT = 'out';
export const BOTH = 'both';

export const files = {
    "test_input.txt": [
        "test_input.txt",
        "test_remove_holes.txt",
        "test_triangulate_EC.txt",
        "test_triangulate_OPT.txt",
        "test_triangulate_MONO_origin_correct.txt",
    ],
    "test_input_hexagon.txt": [
        "test_input_hexagon.txt",
        "test_remove_hexagon_holes.txt",
        "test_triangulate_hexagon_EC.txt",
        "test_triangulate_hexagon_OPT.txt",
        "test_triangulate_hexagon_MONO_origin_correct.txt",
    ],
    "test_input_hexagon_hole.txt": [
        "test_input_hexagon_hole.txt",
        "test_remove_hexagon_hole_holes.txt",
        "test_triangulate_hexagon_hole_EC.txt",
        "test_triangulate_hexagon_hole_OPT.txt",
        "test_triangulate_hexagon_hole_MONO_origin_correct.txt",
    ],
};

export default [
    async function RenderInput(inputFileName: string, dumpFileName: string, canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile(createPathToAsset(inputFileName));
            tester = Tester.from_input_text(inputText);
            tester.draw_polygons(canvasId, IN);
            if ([IN, OUT].includes(verbose)) {
                tester.print(verbose);
            } else if (verbose === BOTH) {
                tester.print(IN);
                tester.print(OUT);
            }
            const dump = tester.dump_polygons(IN, false);
            const outputText = await readFile(createPathToAsset(dumpFileName));
            if (dump.localeCompare(outputText) !== 0) {
                throw `Dump Incorrect!\n\nExpected:\n${outputText}\n\nDump:\n${dump}`;
            }
        } catch (e) {
            throw e;
        } finally {
            if (tester) {
                tester.free();
            }
        }
    },
    async function RemoveHoles(inputFileName: string, dumpFileName: string, canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile(createPathToAsset(inputFileName));
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
            const outputText = await readFile(createPathToAsset(dumpFileName));
            if (dump.localeCompare(outputText) !== 0) {
                throw `Dump Incorrect!\n\nExpected:\n${outputText}\n\nDump:\n${dump}`;
            }
        } catch (e) {
            throw e;
        } finally {
            if (tester) {
                tester.free();
            }
        }
    },
    async function EarClipping(inputFileName: string, dumpFileName: string, canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile(createPathToAsset(inputFileName));
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
            const outputText = await readFile(createPathToAsset(dumpFileName));
            if (dump.localeCompare(outputText) !== 0) {
                throw `Dump Incorrect!\n\nExpected:\n${outputText}\n\nDump:\n${dump}`;
            }
        } catch (e) {
            throw e;
        } finally {
            if (tester) {
                tester.free();
            }
        }
    },
    async function OptimalDP(inputFileName: string, dumpFileName: string, canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile(createPathToAsset(inputFileName));
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
            const outputText = await readFile(createPathToAsset(dumpFileName));
            if (dump.localeCompare(outputText) !== 0) {
                throw `Dump Incorrect!\n\nExpected:\n${outputText}\n\nDump:\n${dump}`;
            }
        } catch (e) {
            throw e;
        } finally {
            if (tester) {
                tester.free();
            }
        }
    },
    async function Monotone(inputFileName: string, dumpFileName: string, canvasId: string, verbose?: string) {
        let tester: Tester;
        try {
            const inputText = await readFile(createPathToAsset(inputFileName));
            tester = Tester.from_input_text(inputText);
            tester.test_monotone();
            tester.draw_polygons(canvasId, OUT);
            if ([IN, OUT].includes(verbose)) {
                tester.print(verbose);
            } else if (verbose === BOTH) {
                tester.print(IN);
                tester.print(OUT);
            }
            const dump = tester.dump_polygons(OUT, false);
            const outputText = await readFile(createPathToAsset(dumpFileName));
            if (dump.localeCompare(outputText) !== 0) {
                throw `Dump Incorrect!\n\nExpected:\n${outputText}\n\nDump:\n${dump}`;
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