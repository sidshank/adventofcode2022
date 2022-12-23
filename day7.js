let lines = require('fs').readFileSync("day7.txt", 'utf-8')
    .split('\n');
let currentPath = [];
let directoryStructure = new Map();
let currentDirTotalFileSize = 0;

function getCurrentPath() {
    let path = "";
    for (let i = 0; i < currentPath.length; i++) {
        path += currentPath[i] + "/";
    }
    return path;
}

for (let i = 0; i < lines.length; i++) {
    let line = lines[i];
    
    let currentPathString = getCurrentPath();
    if (line === "$ ls") {
        continue;
    } else if (line === "$ cd ..") {
        if (!directoryStructure.has(currentPathString)) {
            directoryStructure.set(currentPathString, {
                totalFileSize: currentDirTotalFileSize,
                totalSize: currentDirTotalFileSize,
            });
            currentDirTotalFileSize = 0;
        }
        const sizeOfChildDir = directoryStructure.get(currentPathString).totalSize;
        currentPath.pop();

        currentPathString = getCurrentPath();
        const currentDirInfo = directoryStructure.get(currentPathString);
        currentDirInfo.totalSize += sizeOfChildDir;

    } else if (line.startsWith("$ cd ")) {
        if (line !== "$ cd /" && !directoryStructure.has(currentPathString)) {
            directoryStructure.set(currentPathString, {
                totalFileSize: currentDirTotalFileSize,
                totalSize: currentDirTotalFileSize,
            });
            currentDirTotalFileSize = 0;
        }

        let dir = line.substring(5);
        currentPath.push(dir);
    } else if (line.match(/^\d+/)) {
        let fileSize = parseInt(line, 10);
        currentDirTotalFileSize += fileSize;
    }

}

while (currentPath.length > 1) {
    let currentPathString = getCurrentPath();
    if (!directoryStructure.has(currentPathString)) {
        directoryStructure.set(currentPathString, {
            totalFileSize: currentDirTotalFileSize,
            totalSize: currentDirTotalFileSize,
        });
        currentDirTotalFileSize = 0;
    }
    const sizeOfChildDir = directoryStructure.get(currentPathString).totalSize;
    currentPath.pop();

    currentPathString = getCurrentPath();
    const currentDirInfo = directoryStructure.get(currentPathString);
    currentDirInfo.totalSize += sizeOfChildDir;
}

let totalSize = 0;
directoryStructure.forEach((value, key) => {
    if (value.totalSize <= 100000) {
        totalSize += value.totalSize;
    }
});
// Part 1
console.log(totalSize);

const totalCapacity = 70000000;
const spaceAvailable = totalCapacity - directoryStructure.get("//").totalSize;
const additionalSpaceNeeded = 30000000 - spaceAvailable;

let smallestDir = null;
directoryStructure.forEach((value, key) => {
    if (value.totalSize >= additionalSpaceNeeded) {
        if (smallestDir === null) {
            smallestDir = value;
        } else {
            if (value.totalSize < smallestDir.totalSize) {
                smallestDir = value;
            }
        }
    }
});

// Part 2
console.log(smallestDir.totalSize);