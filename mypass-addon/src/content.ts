const manifest = browser.runtime.getManifest();
console.log(`${manifest.name} ${manifest.version}`);

browser.runtime.onMessage.addListener(pass => {
  setTimeout(() => {
    // execCommandは非推奨らしいけど, 代替手段が無いんだからしょうがない
    document.execCommand("insertText", false, pass);
  }, 300);
});
