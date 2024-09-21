import { BarChart, Bar, XAxis, YAxis, Tooltip, CartesianGrid } from "recharts";
import { format, parseISO } from "date-fns";

type EventCountBucketRow = {
  timestamp_bucket: string;
  cluster_id: string;
  event_count: number;
};

type TimestampPartitionedBuckets = {
  [timestamp_bucket: string]: EventCountBucketRow[];
};

type ClusterKey = `event_count_cluster_${string}`;

type EventCountBucket = {
  timestamp_bucket: string;
  [cluster_id: ClusterKey]: number;
};

type EventCountBucketInfo = {
  eventCountBuckets: EventCountBucket[];
  clusterKeys: ClusterKey[];
};

const getEventCountBuckets = (
  eventCountBucketRows: EventCountBucketRow[]
): EventCountBucketInfo => {
  const partitionedBuckets: TimestampPartitionedBuckets = {};

  // First, partition the rows based on timestamp bucket
  eventCountBucketRows.forEach((row) => {
    const timestamp = row.timestamp_bucket;
    if (timestamp in partitionedBuckets) {
      partitionedBuckets[timestamp].push(row);
    } else {
      partitionedBuckets[timestamp] = [row];
    }
  });

  // Then consolidate rows that should be in the same bucket
  const eventCountBuckets: EventCountBucket[] = [];
  const clusterKeys: ClusterKey[] = [];

  Object.entries(partitionedBuckets).forEach(([timestamp, rows]) => {
    const eventCountBucket: EventCountBucket = {
      timestamp_bucket: timestamp,
    };

    rows.forEach((row) => {
      const clusterKey: ClusterKey = `event_count_cluster_${row.cluster_id}`;
      if (!clusterKeys.includes(clusterKey)) {
        clusterKeys.push(clusterKey);
      }
      eventCountBucket[clusterKey] = row.event_count;
    });

    eventCountBuckets.push(eventCountBucket);
  });

  return { eventCountBuckets, clusterKeys };
};

const StackedBarChart: React.FC<{
  eventCountBucketRows: EventCountBucketRow[];
}> = ({ eventCountBucketRows }) => {
  if (eventCountBucketRows.length == 0) {
    return <></>;
  }

  const { eventCountBuckets, clusterKeys } =
    getEventCountBuckets(eventCountBucketRows);
  console.log(eventCountBuckets);

  const formatXAxis = (tickItem: string) => {
    return format(parseISO(tickItem), "HH:mm");
  };

  const stringToColor = (str: string) => {
    let hash = 0;
    for (let i = 0; i < str.length; i++) {
      hash = str.charCodeAt(i) + ((hash << 5) - hash);
    }
    let color = "#";
    for (let i = 0; i < 3; i++) {
      const value = (hash >> (i * 8)) & 0xff;
      color += ("00" + value.toString(16)).slice(-2);
    }
    return color;
  };

  return (
    <BarChart width={800} height={400} data={eventCountBuckets}>
      <CartesianGrid strokeDasharray="3 3" />
      <XAxis dataKey="timestamp_bucket" tickFormatter={formatXAxis} />
      <YAxis />
      <Tooltip />
      {clusterKeys.map((clusterKey) => (
        <Bar
          key={clusterKey}
          dataKey={clusterKey}
          stackId="constant_id_because_we_want_to_stack"
          fill={stringToColor(clusterKey)}
        />
      ))}
    </BarChart>
  );
};

export default StackedBarChart;
