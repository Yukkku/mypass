const manifest = browser.runtime.getManifest();
console.log(`${manifest.name} ${manifest.version}`);

browser.runtime.onMessage.addListener(info => {
  console.log(info);
});
