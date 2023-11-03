import { fractal_backend } from "../../declarations/fractal_backend";

document.getElementById("button").addEventListener("click", async (e) => {
  e.preventDefault();
  const x = parseInt(document.getElementById("x").value);
  const y = parseInt(document.getElementById("y").value);

  const png = await fractal_backend.fractal(x, y);
  const blob = new Blob([png], { type: "image/png" });
  const url = await convertToDataUrl(blob);

  button.removeAttribute("disabled");
  document.getElementById("fractal").src = url;

  return false;
});

// Converts the given blob into a data url such that it can be assigned as a
// target of a link of as an image source.
function convertToDataUrl(blob) {
  return new Promise((resolve, _) => {
    const fileReader = new FileReader();
    fileReader.readAsDataURL(blob);
    fileReader.onloadend = function () {
      resolve(fileReader.result);
    }
  });
}
