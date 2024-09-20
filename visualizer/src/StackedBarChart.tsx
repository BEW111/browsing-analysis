import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  CartesianGrid,
  Legend,
} from "recharts";

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

// // TODO: fix snake case to camel case
// // TODO: adjust for timezone locale
// const tickFormatter = (timestamp_bucket: string) => {
//   return timestamp_bucket;
// };

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

  const colors = ["#8884d8", "#82ca9d", "#ffc658", "#ff7300", "#a4de6c"]; // Add more colors as needed

  return (
    <BarChart width={800} height={400} data={eventCountBuckets}>
      <CartesianGrid strokeDasharray="3 3" />
      <XAxis dataKey="timestamp_bucket" />
      <YAxis />
      <Tooltip />
      <Legend />
      {clusterKeys.map((clusterKey, index) => (
        <Bar
          key={clusterKey}
          dataKey={clusterKey}
          stackId="a"
          fill={colors[index % colors.length]}
        />
      ))}
    </BarChart>
  );
};

export default StackedBarChart;
