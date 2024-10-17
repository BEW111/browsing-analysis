import ActivityStackedBarCard from "./components/ActivityStackedBarCard";
import ClusterViewCard from "./components/ClusterViewCard";

function App() {
  return (
    <div>
      <div className="h-24 flex align-middle items-center px-12">
        <p className="text-lg">Streams of browsing</p>
      </div>
      <div className="flex gap-8 p-8 flex-wrap">
        <ActivityStackedBarCard />
        <ClusterViewCard />
      </div>
    </div>
  );
}

export default App;
