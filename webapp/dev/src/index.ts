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
        try {
            await fn(testName, BOTH);
            console.log("%c" + testName + " Success!", "color: lime;");
        } catch (e) {
            console.error(e);
            success = false;
        } finally {
            console.groupEnd();
            results[testName] = success? "Success" : "Fail";
        }
    }

    console.table(results);
}

main()
.then(() => console.log("main() finishes."))
.catch(console.error);