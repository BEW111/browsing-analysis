import { useEffect, useState } from "react";
import { BarChart, Card } from "@tremor/react";
import {
  getEventBucketData,
  EventCountBucketInfo,
} from "../utils/eventBucketData";

function ActivityStackedBarCard() {
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
      <Card className="mx-auto max-w-4xl">
        <h4 className="text-tremor-default text-tremor-content dark:text-dark-tremor-content">
          Recently used tabs
        </h4>
        <p className="mb-8 text-tremor-metric font-semibold text-tremor-content-strong dark:text-dark-tremor-content-strong">
          137 events
        </p>
        <div className="flex flex-col gap-16">
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
      </Card>
    );
  }
}

export default ActivityStackedBarCard;
