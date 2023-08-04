import { icptest_backend } from "../../declarations/icptest_backend";

document.getElementById("buttonSet").addEventListener("click", async (e) => {
  try {
    const value = document.getElementById("text").value.toString();
    e.target.setAttribute("disabled", true);
    await icptest_backend.set(value);
  } catch (e) {
    alert(e);
  } finally {
    e.target.removeAttribute("disabled");
  }
  return true;
});

document.getElementById("buttonGet").addEventListener("click", async (e) => {
  try {
    e.target.setAttribute("disabled", true);
    const value = await icptest_backend.get();
    document.getElementById("result").value = value;
  } catch (e) {
    alert(e);
  } finally {
    e.target.removeAttribute("disabled");
  }
  return true;
});
