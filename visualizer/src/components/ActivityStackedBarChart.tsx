import { useEffect, useState } from "react";
import { BarChart } from "@tremor/react";
import {
  getEventBucketData,
  EventCountBucketInfo,
} from "../utils/eventBucketData";

function ActivityStackedBarChart() {
  const [eventBucketData, setEventBucketData] =
    useState<null | EventCountBucketInfo>(null);

  const refreshEventBucketData = async () => {
    const data: EventCountBucketInfo = await getEventBucketData();
    setEventBucketData(data);
  };

  useEffect(() => {
    refreshEventBucketData();
  }, []);

  if (eventBucketData) {
    const { eventCountBuckets, clusterKeys } = eventBucketData;

    return (
      <div className="flex flex-col gap-16 pt-4">
        <BarChart
          stack={true}
          className="h-72"
          data={eventCountBuckets}
          index="timestamp_bucket"
          categories={clusterKeys}
          showLegend={false}
          yAxisLabel="Events"
        />
      </div>
    );
  }
}

export default ActivityStackedBarChart;
