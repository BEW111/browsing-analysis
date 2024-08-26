import { useEffect, useState } from "react";
import "./App.css";
import ActivityLineChart from "./ActivityLineChart";

// type TabUpdateRow = {
//   id: number;
//   timestamp: string;
//   tab_id: number;
//   url: string;
//   title: string;
//   type_of_visit: string;
// };

type TabViewBucketRow = {
  timestamp_bucket: string;
  tab_view_count: number;
};

// type TabUpdate = {
//   timestamp: string;
//   tabId: number;
//   url: string;
//   title: string;
//   typeOfVisit: string;
// };

function App() {
  // const [tabUpdateEvents, setTabUpdateEvents] = useState<TabUpdate[]>([]);
  const [tabViewBuckets, setTabViewBuckets] = useState<TabViewBucketRow[]>([]);

  const refreshTabViewBuckets = async () => {
    const response = await fetch("http://localhost:8000/get_tab_view_buckets");
    const tabViewBucketsJson = await response.json();
    const tabViewBuckets: TabViewBucketRow[] = tabViewBucketsJson.map(
      (row: TabViewBucketRow) => ({
        timestamp_bucket: row.timestamp_bucket,
        tab_view_count: row.tab_view_count,
      })
    );

    setTabViewBuckets(tabViewBuckets);
  };

  // const refreshTabUpdateEvents = async () => {
  //   const response = await fetch("http://localhost:8000/return_all_events");
  //   const tabUpdateEventsJson = await response.json();
  //   const tabUpdateEvents: TabUpdate[] = tabUpdateEventsJson.map(
  //     (row: TabUpdateRow) => ({
  //       timestamp: row.timestamp,
  //       tabId: row.tab_id,
  //       url: row.url,
  //       title: row.title,
  //       typeOfVisit: row.type_of_visit,
  //     })
  //   );

  //   setTabUpdateEvents(tabUpdateEvents);
  // };

  useEffect(() => {
    refreshTabViewBuckets();
    // refreshTabUpdateEvents();
  }, []);

  return (
    <>
      <h1>browsing</h1>
      <ActivityLineChart data={tabViewBuckets} />
    </>
  );
}

export default App;
