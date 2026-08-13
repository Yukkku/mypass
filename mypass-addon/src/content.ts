const manifest = browser.runtime.getManifest();
console.log(`${manifest.name} ${manifest.version}`);

browser.runtime.onMessage.addListener(pass => {
  const elem = document.activeElement;
  console.log(elem);
  if (!(elem instanceof HTMLInputElement)) return;
  elem.value = pass;
});
