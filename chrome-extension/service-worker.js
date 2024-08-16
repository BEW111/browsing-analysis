// Function to log page visits
function logPageVisit(tabId, url, title, typeOfVisit) {
  const visitData = {
    timestamp: new Date().toISOString(),
    tabId: tabId,
    url: url,
    title: title,
    typeOfVisit: typeOfVisit,
  };

  console.log("Page visited:", visitData);
}

// Listen for tab updates
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === "complete" && tab.url) {
    logPageVisit(tabId, tab.url, tab.title, "update");
  }
});

// Listen for tab activation (switching between tabs)
chrome.tabs.onActivated.addListener((activeInfo) => {
  chrome.tabs.get(activeInfo.tabId, (tab) => {
    if (tab.url) {
      logPageVisit(tab.id, tab.url, tab.title, "activate");
    }
  });
});
