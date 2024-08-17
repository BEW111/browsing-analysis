import { useEffect, useState } from "react";
import reactLogo from "./assets/react.svg";
import viteLogo from "/vite.svg";
import "./App.css";

type TabUpdateRow = {
  id: number;
  timestamp: string;
  tab_id: number;
  url: string;
  title: string;
  type_of_visit: string;
};

type TabUpdate = {
  timestamp: string;
  tabId: number;
  url: string;
  title: string;
  typeOfVisit: string;
};

function App() {
  const [tabUpdateEvents, setTabUpdateEvents] = useState<TabUpdate[]>([]);

  const refreshTabUpdateEvents = async () => {
    const response = await fetch("http://localhost:8000/return_all_events");
    const tabUpdateEventsJson = await response.json();
    const tabUpdateEvents: TabUpdate[] = tabUpdateEventsJson.map(
      (row: TabUpdateRow) => ({
        timestamp: row.timestamp,
        tabId: row.tab_id,
        url: row.url,
        title: row.title,
        typeOfVisit: row.type_of_visit,
      })
    );

    setTabUpdateEvents(tabUpdateEvents);
  };

  useEffect(() => {
    refreshTabUpdateEvents();
  }, []);

  return (
    <>
      <div>
        <a href="https://vitejs.dev" target="_blank">
          <img src={viteLogo} className="logo" alt="Vite logo" />
        </a>
        <a href="https://react.dev" target="_blank">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>
      <h1>Browsing Analysis</h1>
      <div>
        {tabUpdateEvents.map((tabUpdateEvent) => (
          <div key={tabUpdateEvent.tabId}>
            <p>{tabUpdateEvent.url}</p>
          </div>
        ))}
      </div>
    </>
  );
}

export default App;
