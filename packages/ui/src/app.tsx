import { useState } from 'react';

import { run } from '@janis.me/calc';

export default function App() {
  const [runResult] = useState(run());
  return (
    <div className="app">
      <h1>Incremental Graph Calc</h1>
      <ul>
        <li>Value: {runResult.value}</li>
        <li>Visited: {runResult.visited.join(', ')}</li>
        <li>Recalculated: {runResult.recalculated.join(', ')}</li>
      </ul>
    </div>
  );
}
