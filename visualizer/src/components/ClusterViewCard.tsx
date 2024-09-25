import { useEffect, useState } from "react";
import {
  Card,
  List,
  ListItem,
  SearchSelect,
  SearchSelectItem,
} from "@tremor/react";
import { Cluster, getClusters } from "../utils/clusterData";
import { getPages } from "../utils/pageData";

function ClusterViewCard() {
  const [clusterData, setClusterData] = useState<null | Cluster[]>();
  const [pageUrls, setPageUrls] = useState<null | string[]>();

  const refreshClusterData = async () => {
    const data: Cluster[] = await getClusters();
    setClusterData(data);
  };

  const onSetClusterId = async (clusterId: string) => {
    const pageUrls: string[] = await getPages(clusterId);
    setPageUrls(pageUrls);
  };

  useEffect(() => {
    refreshClusterData();
  }, []);

  if (clusterData && clusterData.length > 0) {
    return (
      <Card className="mx-auto max-w-4xl">
        <h4 className="text-tremor-default text-tremor-content dark:text-dark-tremor-content">
          Pages by cluster
        </h4>
        <p className="text-tremor-metric font-semibold text-tremor-content-strong dark:text-dark-tremor-content-strong">
          {pageUrls ? pageUrls.length : 0} pages
        </p>
        <SearchSelect className="my-4" onValueChange={onSetClusterId}>
          {clusterData.map((cluster) => (
            <SearchSelectItem key={cluster.id} value={cluster.id}>
              {cluster.name}
            </SearchSelectItem>
          ))}
        </SearchSelect>
        {pageUrls && (
          <List className="mx-auto max-w-md">
            {pageUrls.map((pageUrl) => (
              <ListItem key={pageUrl}>
                <a
                  href={pageUrl}
                  className="text-sm text-tremor-content-emphasis dark:text-dark-tremor-content-emphasis"
                >
                  {pageUrl}
                </a>
              </ListItem>
            ))}
          </List>
        )}
      </Card>
    );
  }
}

export default ClusterViewCard;
