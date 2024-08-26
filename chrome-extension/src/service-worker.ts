type BrowseEventType = "activate" | "update";

const BACKEND_SERVER_URL = "http://localhost:8000";

function sendBrowseEvent(
  tabId: number,
  pageUrl: string,
  pageTitle: string,
  pageContent: string | null,
  eventType: BrowseEventType
) {
  const browseEvent = {
    tab_id: tabId,
    timestamp: new Date().toISOString(),
    page_url: pageUrl,
    page_title: pageTitle,
    page_content: pageContent,
    event_type: eventType,
  };

  console.log(`Sending event ${browseEvent}`);

  // TODO: update server url here
  fetch(`${BACKEND_SERVER_URL}/log_event`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body: JSON.stringify(browseEvent),
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

// This function can only be run via the scripting api
function getPageContent() {
  return document.body.innerHTML;
}

// Tab updates (changing the contents of the tab, e.g. clicking a link)
chrome.tabs.onUpdated.addListener((tabId, changeInfo, tab) => {
  if (changeInfo.status === "complete") {
    chrome.scripting
      .executeScript({
        target: { tabId: tabId },
        func: getPageContent,
      })
      .then((pageContentResults) => {
        if (pageContentResults.length > 0) {
          let pageContent = pageContentResults[0].result;
          if (pageContent && tab.url && tab.title) {
            sendBrowseEvent(tabId, tab.url, tab.title, pageContent, "update");
          }
        }
      });
  }
});

// Tab activation (clicking on a tab, also creating)
chrome.tabs.onActivated.addListener((activeInfo) => {
  chrome.tabs.get(activeInfo.tabId, (tab) => {
    if (tab.id && tab.url && tab.title) {
      sendBrowseEvent(tab.id, tab.url, tab.title, null, "activate");
    }
  });
});

// TODO: Window focus
chrome.windows.onFocusChanged.addListener((windowId) => {
  if (windowId === chrome.windows.WINDOW_ID_NONE) {
    console.log("Chrome window lost focus");
  } else {
    console.log("Chrome window gained focus");
  }
});

// TODO: Chrome closing
chrome.runtime.onSuspend.addListener(() => {
  console.log("Chrome is closing");
});
