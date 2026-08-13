import init, { generate } from "mypass-wasm";
import wasm from "mypass-wasm/mypass_wasm_bg.wasm";

init(wasm).then(() => {
  browser.contextMenus.create({
    id: 'req-mypass',
    title: 'mypass',
    contexts: ['editable'],
  });

  browser.contextMenus.onClicked.addListener((info, tab) => {
    if (info.menuItemId !== 'req-mypass') return;
    if (tab?.id == null) return;
    browser.tabs.sendMessage(tab.id, generate({ len: 100 }, "scratch", new Uint8Array()));
  });
});
