import { useState } from 'react';

import { run } from '@janis.me/calc';

export default function App() {
  const [runResult] = useState(run());
  return (
    <div className="app">
      <h1>Incremental Graph Calc</h1>
      <p>Result:</p>
      <pre>
        {JSON.stringify(
          {
            nodesProcessed: runResult.nodes_processed,
            durationMs: runResult.duration_ms,
            iterations: runResult.iterations,
            outputValue: runResult.output_value,
            firstError: runResult.first_error ? runResult.first_error.toString() : null,
            errorNode: runResult.error_node ? runResult.error_node.toString() : null,
            aborted: runResult.aborted,
          },
          null,
          2,
        )}
      </pre>
    </div>
  );
}
