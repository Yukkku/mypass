import "./popup.css" with { type: "css" };

const input = document.createElement("input");
input.type = "password";
input.style.width = "30em";

input.addEventListener("keydown", e => {
  if (e.key !== "Enter") return;
  browser.runtime.sendMessage({
    type: "phrase",
    phrase: input.value,
  });
  window.close();
});

document.body.appendChild(input);
document.addEventListener("DOMContentLoaded", () => {
  input.focus();
});
