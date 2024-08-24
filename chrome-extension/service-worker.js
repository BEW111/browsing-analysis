// Function to log page visits
function logPageVisit(tabId, url, title, typeOfVisit) {
  const tabUpdateEvent = {
    tab_id: tabId,
    timestamp: new Date().toISOString(),
    title: title,
    type_of_visit: typeOfVisit,
    url: url,
  };

  console.log("Page visited:", tabUpdateEvent);

  // TODO: update server url here
  fetch("http://localhost:8000/log_event", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(tabUpdateEvent),
  })
    .then((response) => {
      if (!response.ok) {
        response.text().then((errorText) => {
          throw new Error(`(status: ${response.status}) ${errorText}`);
        });
      } else {
        return response.json();
      }
    })
    .catch((error) => {
      console.error("Error:", error);
    });
}

// Tab updates (changing the contents of the tab, e.g. clicking a link)
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === "complete" && tab.url) {
    logPageVisit(tabId, tab.url, tab.title, "update");
  }
});

// Tab activation (clicking on a tab, also creating)
chrome.tabs.onActivated.addListener((activeInfo) => {
  chrome.tabs.get(activeInfo.tabId, (tab) => {
    if (tab.url) {
      logPageVisit(tab.id, tab.url, tab.title, "activate");
    }
  });
});

// Window focus
chrome.windows.onFocusChanged.addListener((windowId) => {
  if (windowId === chrome.windows.WINDOW_ID_NONE) {
    console.log("Chrome window lost focus");
  } else {
    console.log("Chrome window gained focus");
  }
});

// Chrome closing
chrome.runtime.onSuspend.addListener(() => {
  console.log("Chrome is closing");
  // Perform any cleanup or final logging here
});
