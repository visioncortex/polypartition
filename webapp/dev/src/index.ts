import testFunctions, { IN, OUT, BOTH } from "./tests";

async function main() {
    let results: {[k: string]: string} = {};
    
    for (const key in testFunctions) {
        const fn = testFunctions[key];
        const testName = fn.name;
        let success = true;

        // Create the HTML row in the table
        const tr = document.createElement('tr');
        const nameTd = document.createElement('td');
        const canvasTd = document.createElement('td');
        nameTd.innerHTML = testName;
        canvasTd.innerHTML = `<canvas id="${testName}" width="352" height="416"></canvas>`;
        tr.appendChild(nameTd);
        tr.appendChild(canvasTd);
        (document.getElementById('results_table') as HTMLTableElement).tBodies[0].appendChild(tr);

        console.groupCollapsed(testName);
        let t0, t1;
        try {
            t0 = performance.now();
            await fn(testName, BOTH);
            t1 = performance.now();
            console.log("%c" + testName + " Success!", "color: lime;");
        } catch (e) {
            console.error(e);
            success = false;
        } finally {
            console.groupEnd();
            results[testName] = success? "Success in " + (t1 - t0).toFixed(2) + " ms" : "Fail";
        }
    }

    console.table(results);
}

main()
.then(() => console.log("main() finishes."))
.catch(console.error);