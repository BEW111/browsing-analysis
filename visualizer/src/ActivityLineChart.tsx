import { BarChart, Bar, XAxis, YAxis, Tooltip } from "recharts";

type TabViewBucketRow = {
  timestamp_bucket: string;
  tab_view_count: number;
};

// TODO: fix snake case to camel case
// TODO: adjust for timezone locale
const tickFormatter = (timestamp_bucket: string) => {
  return timestamp_bucket.split(" ")[1];
};

const ActivityLineChart = ({ data }: { data: TabViewBucketRow[] }) => {
  console.log(data);

  return (
    data.length > 0 && (
      <div>
        <BarChart width={1200} height={600} data={data}>
          <Bar type="monotone" dataKey="tab_view_count" stroke="#8884d8" />
          <XAxis dataKey="timestamp_bucket" tickFormatter={tickFormatter} />
          <YAxis />
          <Tooltip />
        </BarChart>
      </div>
    )
  );
};

export default ActivityLineChart;
