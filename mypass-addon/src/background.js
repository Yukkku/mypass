browser.contextMenus.create({
  id: 'mypass',
  title: 'mypass',
  contexts: ['editable'],
});

import wasm from "mypass-wasm/mypass_wasm_bg.wasm";
import init, { generate } from "mypass-wasm";

let msg = 0;

browser.contextMenus.onClicked.addListener((_info, tab) => {
  browser.tabs.sendMessage(tab.id, msg);
});

init(wasm).then(() => {
  msg = [0];
  const pass = generate({ len: 100 }, "scratch", new Uint8Array());
  msg = [1, pass];
}).catch(e => {
  msg = e;
});
