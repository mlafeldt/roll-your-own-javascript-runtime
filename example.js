console.log("Hello", "Deno!");
console.error("Boom!");

const path = "./log.txt";
try {
  const contents = await Deno.readTextFile(path);
  console.log("Read from a file", contents);
} catch (err) {
  console.error("Unable to read file", path, err);
}

await Deno.writeTextFile(path, "I can write to a file.");
const contents = await Deno.readTextFile(path);
console.log("Read from a file", path, "contents:", contents);
console.log("Removing file", path);
Deno.remove(path);
console.log("File removed");
