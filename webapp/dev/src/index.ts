import testFunctions, { IN, OUT, BOTH } from "./tests";

async function main() {
    let results: {[k: string]: string} = {};
    
    for (const key in testFunctions) {
        const fn = testFunctions[key];
        const testName = fn.name;
        let success = true;

        console.groupCollapsed(testName);
        try {
            await fn('main');
            console.log("%c" + testName + " Success!", "color: lime;");
        } catch (e) {
            console.error(e);
            success = false;
        }
        console.groupEnd();

        results[testName] = success? "Success" : "Fail";
    }

    console.table(results);
}

main()
.then(() => console.log("main() finishes."))
.catch(console.error);