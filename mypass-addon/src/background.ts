import init, { generate } from "mypass-wasm";
import wasm from "mypass-wasm/mypass_wasm_bg.wasm";

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
    if (msg.type !== "password") return;
    const password: string = msg.password;
    const tabs = await browser.tabs.query({ currentWindow: true, active: true });;
    console.log(tabs);
    for (const tab of tabs) {
      if (tab.id == null) continue;
      if (tab.url == null) continue;
      browser.tabs.sendMessage(tab.id, generate({ len: 100 }, new URL(tab.url).hostname, new Uint8Array()));
    }
  });
});
