type PageUrlRow = {
  url: string;
};

const getPages = async (clusterId: string) => {
  const response = await fetch(
    `http://localhost:8000/get_pages?cluster_id=${clusterId}`
  );
  const data: PageUrlRow[] = await response.json();
  const pageUrls: string[] = data.map((row) => row.url);

  return pageUrls;
};

export { getPages };
