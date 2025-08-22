import { useEffect, useState, useSyncExternalStore } from 'react';

import { DerivedNode, Graph, InputNode, OutputNode } from '@janis.me/calc';

export default function useGraph() {
  const [graph, setGraph] = useState(new Graph());

  useEffect(() => {
    if (graph) {
      graph.addInput(new InputNode(2)); // 0
      graph.addInput(new InputNode(3)); // 1
      graph.addDerived(new DerivedNode(new Int32Array([0, 1]), 'Add')); //2
      graph.addOutput(new OutputNode(2)); //3

      graph.run();

      graph.subscribeToNodes((value: unknown) => {
        console.log('Graph updated');
        console.log(value);
      });
    }
  }, []);

  return [];
}
