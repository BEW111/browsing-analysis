import {
  BarChart,
  Bar,
  XAxis,
  YAxis,
  Tooltip,
  CartesianGrid,
  TooltipProps,
} from "recharts";
import { format, parseISO } from "date-fns";

type EventCountBucketRow = {
  timestamp_bucket: string;
  cluster_id: string;
  cluster_name: string | null;
  event_count: number;
};

type TimestampPartitionedBuckets = {
  [timestamp_bucket: string]: EventCountBucketRow[];
};

type ClusterKey = `event_count_cluster_${string}`;

type PayloadValue = {
  cluster_name: string | null;
  value: number;
};

type EventCountBucket = {
  timestamp_bucket: string;
  [cluster_id: ClusterKey]: PayloadValue;
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
      eventCountBucket[clusterKey] = {
        cluster_name: row.cluster_name,
        value: row.event_count,
      };
    });

    eventCountBuckets.push(eventCountBucket);
  });

  return { eventCountBuckets, clusterKeys };
};

const CustomTooltip = ({
  active,
  payload,
  label,
}: TooltipProps<number, string>) => {
  if (active && payload && payload.length) {
    const currentBarPayload = payload[0].payload;

    return (
      <div
        className="custom-tooltip"
        style={{
          textAlign: "left",
          backgroundColor: "rgba(0.9, 0.9, 0.9, 0.1)",
        }}
      >
        {Object.entries(currentBarPayload).map(([clusterId, clusterInfo]) => (
          <div>
            <p className="desc" style={{ fontSize: 12, color: "white" }}>
              {clusterInfo.cluster_name || clusterId}: {clusterInfo.value}
            </p>
          </div>
        ))}
      </div>
    );
  }

  return null;
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
      <Tooltip content={<CustomTooltip />} />
      {clusterKeys.map((clusterKey) => (
        <Bar
          key={clusterKey}
          dataKey={(item) => item[clusterKey]?.value || 0}
          stackId="constant_id_because_we_want_to_stack"
          fill={stringToColor(clusterKey)}
        />
      ))}
    </BarChart>
  );
};

export default StackedBarChart;
