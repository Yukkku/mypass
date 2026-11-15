const manifest = browser.runtime.getManifest();
console.log(`${manifest.name} ${manifest.version}`);

browser.runtime.onMessage.addListener(pass => {
  document.execCommand("insertText", false, pass);
});
