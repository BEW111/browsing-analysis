import { useEffect, useState } from "react";
import "./App.css";
import StackedBarChart from "./StackedBarChart";

type EventCountBucketRow = {
  timestamp_bucket: string;
  cluster_id: string;
  cluster_name: string | null;
  event_count: number;
};

function App() {
  const [eventCountBuckets, setEventCountBuckets] = useState<
    EventCountBucketRow[]
  >([]);

  const refreshTabViewBuckets = async () => {
    const response = await fetch("http://localhost:8000/get_event_buckets");
    const eventCountBucketsJson = await response.json();
    const eventCountBuckets: EventCountBucketRow[] = eventCountBucketsJson.map(
      (row: EventCountBucketRow) => ({
        timestamp_bucket: row.timestamp_bucket,
        cluster_id: row.cluster_id,
        cluster_name: row.cluster_name,
        event_count: row.event_count,
      })
    );

    setEventCountBuckets(eventCountBuckets);
  };

  useEffect(() => {
    refreshTabViewBuckets();
  }, []);

  return (
    <>
      <h1>browsing</h1>
      <StackedBarChart eventCountBucketRows={eventCountBuckets} />
    </>
  );
}

export default App;
