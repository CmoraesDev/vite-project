import { readFile, writeFile, writeFileSync, promises as fsPromises } from 'fs';
// const {readFile, writeFile, promises: fsPromises} = require('fs');

function replaceInfile(filename, projectName) {
    console.log("Switching name in file " + filename)
    readFile(filename, 'utf-8', function (err, contents) {
        if (err) {
            console.log(err);
            return;
        }

        const replaced = contents.replace(/vite-project/g, projectName);

        writeFile(filename, replaced, 'utf-8', function (err) {
            console.log(err);
        });
    });
}

function setupProject(projectName) {
    console.log("Seting up project " + projectName);

    const files = [
        "./package.json",
        "./package-lock.json",
        "./src/stores/GlobalTheme.ts",
        "./src-tauri/tauri.conf.json"
    ];

    files.forEach((filename) => {
        replaceInfile(filename, projectName)
    })

}

if (process.argv.length != 3) {
    console.log("Usage: node setup.js <ProjectName>")
    process.exit(1)
}


let projectName = process.argv[2]

setupProject(projectName)
