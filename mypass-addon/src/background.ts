import init, { generate } from "mypass-wasm";
import wasm from "mypass-wasm/mypass_wasm_bg.wasm";

interface ConfigFile {
  services: {
    [service: string]: {
      len: number;
      allow_chars?: string;
      requires?: string[];
      info?: string;
    }
  }
}

const getConfig = async () => {
  const { config } = await browser.storage.local.get(["config"]);
  if (config != null) {
    return config as ConfigFile;
  } else {
    throw new Error("masterpass is not registered");
  }
};

const getMasterpass = async () => {
  const { masterpass } = await browser.storage.session.get(["masterpass"]);
  if (masterpass instanceof Uint8Array) {
    return masterpass;
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
    const [tabs, masterpass, config] = await Promise.all([
      browser.tabs.query({ currentWindow: true, active: true }),
      getMasterpass(),
      getConfig(),
    ]);
    for (const tab of tabs) {
      if (tab.id == null) continue;
      if (tab.url == null) continue;
      const service = new URL(tab.url).hostname;
      const conf = config.services[service];
      if (!conf) continue;
      browser.tabs.sendMessage(tab.id, generate(
        conf,
        service,
        phrase,
        masterpass,
      ));
    }
  });
});
