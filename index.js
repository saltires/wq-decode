const js = import("./pkg/hello_world");
const data = "3,228,184,173,3,228,184,173,1,97,1,48"
js.then(js => {
  js.greet("WebAssembly");
  console.log(js.bar(data))
});