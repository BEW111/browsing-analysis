type Cluster = {
  id: string;
  name: string;
  clusteringRunId: number;
};

type ClusterRow = {
  id: string;
  name: string;
  clustering_run_id: number;
};

const getClusters = async () => {
  const response = await fetch("http://localhost:8000/get_clusters");
  const data = await response.json();
  const clusters: Cluster[] = data.map((row: ClusterRow) => ({
    id: row.id,
    name: row.name,
    clusteringRunId: row.clustering_run_id,
  }));

  return clusters;
};

export { getClusters, type Cluster };
