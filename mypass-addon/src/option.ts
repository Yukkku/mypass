import "./option.css" with { type: "css" };

const input = document.createElement("input");
input.type = "file";
input.style.marginLeft = "0.5em";

input.addEventListener("change", async () => {
  if (input.files!.length == 0) return;
  const file = input.files![0]!;
  const masterpass = await file.bytes();
  browser.storage.session.set({ masterpass });
});

const label = document.createElement("label");
label.textContent = "Masterpass:";
label.appendChild(input);

document.body.appendChild(label);
