type EventCountBucketRow = {
  timestamp_bucket: string;
  cluster_id: string;
  cluster_name: string | null;
  event_count: number;
};

type TimestampPartitionedBuckets = {
  [timestamp_bucket: string]: EventCountBucketRow[];
};

type ClusterKey = `Cluster: ${string}`;

type EventCountBucket = {
  timestamp_bucket: string;
  [cluster_id: ClusterKey]: number;
};

type EventCountBucketInfo = {
  eventCountBuckets: EventCountBucket[];
  clusterKeys: ClusterKey[];
};

type ClusteringRunRow = {
  clustering_run: string;
};

const getClusteringRuns = async () => {
  const response = await fetch("http://localhost:8000/get_clustering_runs");
  const clusteringRunsJson = await response.json();
  const clustering_runs: string[] = clusteringRunsJson.map(
    (row: ClusteringRunRow) => row.clustering_run
  );
  return clustering_runs;
};

const getEventCountBucketRows = async (clustering_run: string) => {
  const response = await fetch(
    `http://localhost:8000/get_event_buckets?clustering_run=${clustering_run}`
  );
  const eventCountBucketsJson = await response.json();
  const eventCountBuckets: EventCountBucketRow[] = eventCountBucketsJson.map(
    (row: EventCountBucketRow) => ({
      timestamp_bucket: row.timestamp_bucket,
      cluster_id: row.cluster_id,
      cluster_name: row.cluster_name,
      event_count: row.event_count,
    })
  );

  return eventCountBuckets;
};

const partitionEventCountBuckets = (
  eventCountBucketRows: EventCountBucketRow[]
) => {
  const partitionedBuckets: TimestampPartitionedBuckets = {};

  eventCountBucketRows.forEach((row) => {
    const timestamp = row.timestamp_bucket;
    if (timestamp in partitionedBuckets) {
      partitionedBuckets[timestamp].push(row);
    } else {
      partitionedBuckets[timestamp] = [row];
    }
  });

  return partitionedBuckets;
};

const formatTimestamp = (timestamp: string) => {
  const date = new Date(timestamp);
  return date.toLocaleString("en-US", {
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
    hour12: false,
  });
};

const formatEventBucketData = (
  partitionedBuckets: TimestampPartitionedBuckets
) => {
  const eventCountBuckets: EventCountBucket[] = [];
  const clusterKeys: ClusterKey[] = [];

  Object.entries(partitionedBuckets).forEach(([timestamp, rows]) => {
    const eventCountBucket: EventCountBucket = {
      timestamp_bucket: formatTimestamp(timestamp),
    };

    rows.forEach((row) => {
      const clusterKey: ClusterKey = row.cluster_name
        ? `Cluster: ${row.cluster_name.split(" ").slice(0, 3).join(", ")}`
        : `Cluster: ${row.cluster_id}`;
      if (!clusterKeys.includes(clusterKey)) {
        clusterKeys.push(clusterKey);
      }
      eventCountBucket[clusterKey] = row.event_count;
    });

    eventCountBuckets.push(eventCountBucket);
  });

  return { eventCountBuckets, clusterKeys };
};

const addMissingHours = (
  eventCountBuckets: EventCountBucket[]
): EventCountBucket[] => {
  if (eventCountBuckets.length === 0) return [];

  const sortedBuckets = [...eventCountBuckets].sort(
    (a, b) =>
      new Date(a.timestamp_bucket).getTime() -
      new Date(b.timestamp_bucket).getTime()
  );

  const start = new Date(sortedBuckets[0].timestamp_bucket);
  const end = new Date(
    sortedBuckets[sortedBuckets.length - 1].timestamp_bucket
  );
  const allHours: EventCountBucket[] = [];

  for (let d = new Date(start); d <= end; d.setHours(d.getHours() + 1)) {
    const formattedTimestamp = formatTimestamp(d.toISOString());
    const existingBucket = sortedBuckets.find(
      (bucket) => bucket.timestamp_bucket === formattedTimestamp
    );

    if (existingBucket) {
      allHours.push(existingBucket);
    } else {
      const newBucket: EventCountBucket = {
        timestamp_bucket: formattedTimestamp,
      };
      allHours.push(newBucket);
    }
  }

  return allHours;
};

const getEventBucketData = async (clustering_run: string) => {
  const eventCountBucketRows = await getEventCountBucketRows(clustering_run);
  const partitionedBuckets = partitionEventCountBuckets(eventCountBucketRows);
  const { eventCountBuckets, clusterKeys } =
    formatEventBucketData(partitionedBuckets);

  const completeEventCountBuckets = addMissingHours(eventCountBuckets);

  return {
    eventCountBuckets: completeEventCountBuckets,
    clusterKeys,
  };
};

export { getClusteringRuns, getEventBucketData, type EventCountBucketInfo };
