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

  fetch("http://localhost:8000/log_event", {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(tabUpdateEvent),
  })
    .then((response) => {
      if (!response.ok) {
        throw new Error(`HTTP error! status: ${response.status}`);
      }
      return response.json();
    })
    .then((data) => {
      console.log("Success:", data);
    })
    .catch((error) => {
      console.error("Error:", error);
    });
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
