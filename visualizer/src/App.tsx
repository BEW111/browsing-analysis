import { Card } from "@tremor/react";
import ActivityStackedBarChart from "./components/ActivityStackedBarChart";

function App() {
  return (
    <div className="p-36">
      <Card className="mx-auto max-w-4xl">
        <h4 className="text-tremor-default text-tremor-content dark:text-dark-tremor-content">
          Your Browsing Activity
        </h4>
        <p className="text-tremor-metric font-semibold text-tremor-content-strong dark:text-dark-tremor-content-strong">
          XX hrs
        </p>
        <ActivityStackedBarChart />
      </Card>
    </div>
  );
}

export default App;
