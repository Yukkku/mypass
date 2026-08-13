import "./option.css" with { type: "css" };

const masterpass = (() => {
  const input = document.createElement("input");
  input.type = "file";
  input.style.marginLeft = "0.5em";

  input.addEventListener("change", async () => {
    if (input.files!.length == 0) return;
    const file = input.files![0]!;
    const masterpass = await file.arrayBuffer();
    browser.storage.local.set({ masterpass });
  });

  const label = document.createElement("label");
  label.textContent = "Masterpass:";
  label.appendChild(input);
  return label;
})();

const config = (() => {
  const input = document.createElement("input");
  input.type = "file";
  input.style.marginLeft = "0.5em";

  input.addEventListener("change", async () => {
    if (input.files!.length == 0) return;
    const file = input.files![0]!;
    const config = JSON.parse(await file.text());
    browser.storage.local.set({ config });
  });

  const label = document.createElement("label");
  label.textContent = "Config:";
  label.appendChild(input);
  return label;
})();

document.body.append(masterpass, config);
