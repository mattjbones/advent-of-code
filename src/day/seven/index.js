const fs = require('fs');
const util = require('util');

const TARGET_SIZE = 100_000;
const TOTAL_SPACE = 70_000_000;
const MIN_UNUSED_SPACE = 30_000_000

function buildDirStructure(data) {
    let root = {
        path: "/",
        contents: [],
        parent: undefined,
        type: "dir",
        fileSize: 0
    };

    let currentNode = root;
    data.split('\n').forEach(line => {
        // console.log({line});
        const parts = line.split(" ") 
        if (line.startsWith("$")) {
            const [,command, argument] = parts;
            // console.log({command, argument});
            if (command === "cd" ) {
                if (argument === "/") {
                    currentNode = root;
                }else if ( argument  === "..") {
                    currentNode = currentNode.parent;
                }else{
                    let newNode = {
                        path: argument, 
                        contents: [],
                        parent: currentNode,
                        type: "dir",
                        fileSize: 0
                    }
                    currentNode.contents.push(newNode);
                    currentNode = newNode;
                }
                
            }
        }else if(line.startsWith("dir")) {
            const [_, dirName] = parts;
        }else {
            const [fileSize, filename] = parts;
            let numSize = parseInt(fileSize, 10)
            currentNode.contents.push({type: "file", filename, fileSize: numSize});
            currentNode.fileSize += numSize;

        }
    });
    // console.log(util.inspect(root, false, 8, true));
    return root;
}

function calculateDirTotals(rootNode){
    let totals = [];
    function totalWalker(node) {

        if (!node.contents.length) {
            return 0
        }

        const dirs = node.contents.filter(item => item.type === "dir");
        if(dirs.length) {
            const subDirsTotals = dirs.map(dir=> totalWalker(dir));
            const subDirsTotal = subDirsTotals.reduce((acc,val) => acc + val, 0);
            const nodeTotal = subDirsTotal + node.fileSize;
            node.nodeTotal = nodeTotal;
            totals.push(nodeTotal);
            return nodeTotal
        }else{
            totals.push(node.fileSize);
            return node.fileSize;
        }
    }
    totalWalker(rootNode);

    console.log({totals});
    return totals
}

function calculateDirsWithTarget(data) {
    const root = buildDirStructure(data);
    const totals = calculateDirTotals(root);
    const total = totals.filter(total => total<TARGET_SIZE).reduce((prev, curr) => prev+curr);
    console.log({total});
}

function findDirToDelet(data) {
    const root = buildDirStructure(data);
    const totals = calculateDirTotals(root);
    const usedSpace = root.nodeTotal;
    const free = TOTAL_SPACE - usedSpace;
    const target = totals.filter(total => total + free > MIN_UNUSED_SPACE).sort()[0]
    console.log({target});

}

fs.readFile('sample','utf8', (err, data) => {
    console.log("Part 1 - sample");
    if (err) {
        console.error(err)
        return
    }
    calculateDirsWithTarget(data);
    console.log("expected: 95437");
});

fs.readFile('input','utf8', (err, data) => {
    console.log("Part 1 - input");
    if (err) {
        console.error(err)
        return
    }

    calculateDirsWithTarget(data);
    console.log("expected: 1490523")
});


fs.readFile('sample','utf8', (err, data) => {
    console.log("\nPart 2 - sample");
    if (err) {
        console.error(err)
        return
    }
    findDirToDelet(data);
    console.log("prev: 24933642");
});

fs.readFile('input','utf8', (err, data) => {
    console.log("\nPart 2 - input");
    if (err) {
        console.error(err)
        return
    }
    findDirToDelet(data);
    console.log("prev: non");
});