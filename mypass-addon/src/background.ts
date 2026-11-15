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
  return (config ?? {}) as ConfigFile;
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

  let lastTab: browser.tabs.Tab | undefined = undefined;
  browser.contextMenus.onClicked.addListener((info, tab) => {
    if (info.menuItemId !== "req-mypass") return;
    lastTab = tab;
    browser.browserAction.openPopup();
  });

  browser.runtime.onMessage.addListener(async (msg) => {
    if (!lastTab) return;
    if (msg.type !== "phrase") return;
    const phrase: string = msg.phrase;
    const [masterpass, config] = await Promise.all([
      getMasterpass(),
      getConfig(),
    ]);
    if (lastTab.id == null) return;
    if (lastTab.url == null) return;
    const service = new URL(lastTab.url).hostname;
    const conf = config.services[service];
    if (!conf) return;
    browser.tabs.sendMessage(lastTab.id, generate(
      conf,
      service,
      phrase,
      masterpass,
    ));
  });
});
