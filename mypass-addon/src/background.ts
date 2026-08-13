import init, { } from "mypass-wasm";
import wasm from "mypass-wasm/mypass_wasm_bg.wasm";

init(wasm).then(() => {
  browser.contextMenus.create({
    id: 'req-mypass',
    title: 'mypass',
    contexts: ['editable'],
  });

  browser.contextMenus.onClicked.addListener((info, _tab) => {
    if (info.menuItemId !== 'req-mypass') return;
    browser.action.openPopup();
  });
});
