browser.contextMenus.create({
  id: 'mypass',
  title: 'mypass',
  contexts: ['editable'],
});

browser.contextMenus.onClicked.addListener(async (info, tab) => {
  console.log(info, tab);
});
