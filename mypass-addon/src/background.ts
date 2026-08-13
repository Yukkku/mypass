import init, { generate } from "mypass-wasm";
import wasm from "mypass-wasm/mypass_wasm_bg.wasm";

const getMasterpass = async () => {
  const { masterpass } = await browser.storage.local.get(["masterpass"]);
  if (masterpass instanceof ArrayBuffer) {
    return new Uint8Array(masterpass);
  } else {
    throw new Error("masterpass is not registered");
  }
};

init(wasm).then(() => {
  browser.contextMenus.create({
    id: "req-mypass",
    title: "mypass",
    contexts: ["editable"],
  });

  browser.contextMenus.onClicked.addListener((info, _tab) => {
    if (info.menuItemId !== "req-mypass") return;
    browser.action.openPopup();
  });

  browser.runtime.onMessage.addListener(async (msg) => {
    if (msg.type !== "phrase") return;
    const phrase: string = msg.phrase;
    const [tabs, masterpass] = await Promise.all([
      browser.tabs.query({ currentWindow: true, active: true }),
      getMasterpass(),
    ]);
    for (const tab of tabs) {
      if (tab.id == null) continue;
      if (tab.url == null) continue;
      browser.tabs.sendMessage(tab.id, generate(
        { len: 100 },
        new URL(tab.url).hostname,
        phrase,
        masterpass,
      ));
    }
  });
});
