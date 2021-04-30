import testFunctions, { IN, OUT, BOTH, files } from "./tests";

async function main() {
    let results: {[k: string]: string} = {};
    
    for (const [inputFile, dumpFiles] of Object.entries(files)) {
        if (dumpFiles.length != testFunctions.length) {
            throw "Number of dump files specified does not match number of test functions.";
        }

        for (const i in testFunctions) {
            const fn = testFunctions[i];
            const testName = fn.name;
            const key = inputFile + "_" + testName;
            let success = true;
            let errmsg = "Fail: ";

            // Create the HTML row in the table
            const tr = document.createElement('tr');
            const nameTd = document.createElement('td');
            const canvasTd = document.createElement('td');
            nameTd.innerHTML = key;
            canvasTd.innerHTML = `<canvas id="${key}" width="352" height="416"></canvas>`;
            tr.appendChild(nameTd);
            tr.appendChild(canvasTd);
            (document.getElementById('results_table') as HTMLTableElement).tBodies[0].appendChild(tr);

            console.groupCollapsed(key);
            let t0, t1;
            try {
                t0 = performance.now();
                await fn(inputFile, dumpFiles[i], key, BOTH);
                t1 = performance.now();
                console.log("%c" + testName + " Success!", "color: lime;");
            } catch (e) {
                console.error(e);
                success = false;
                errmsg += e;
            } finally {
                console.groupEnd();
                results[key] = success? "Success in " + (t1 - t0).toFixed(2) + " ms" : errmsg;
            }
        }
    }

    console.table(results);
}

main()
.then(() => console.log("main() finishes."))
.catch(console.error);