import {
  addEdge,
  applyEdgeChanges,
  applyNodeChanges,
  Background,
  BackgroundVariant,
  Panel,
  ReactFlow,
} from '@xyflow/react';
import { useCallback, useState } from 'react';

import { DerivedNode, Graph, InputNode, OutputNode } from '@janis.me/calc';

import '@xyflow/react/dist/style.css';

import useGraph from '#hooks/useGraph.js';

const initialNodes = [
  { id: 'n1', position: { x: 0, y: 0 }, data: { label: 'Node 1' } },
  { id: 'n2', position: { x: 0, y: 100 }, data: { label: 'Node 2' } },
];
const initialEdges = [{ id: 'n1-n2', source: 'n1', target: 'n2' }];

export default function App() {
  const nodes = useGraph();

  return <pre>{JSON.stringify(nodes, null, 2)}</pre>;
}
// export default function App() {
//   useEffect(() => {
//     const graph = new Graph();

//     // Create input nodes
//     const inputA = new InputNode(2);
//     const inputB = new InputNode(3);

//     // Create derived node
//     const derivedNode = new DerivedNode(new Int32Array([0, 1]), 'Add');

//     // Create output node
//     const outputNode = new OutputNode(2);

//     // Add nodes to the graph
//     graph.add_input(inputA);
//     graph.add_input(inputB);
//     graph.add_derived(derivedNode);
//     graph.add_output(outputNode);

//     // Subscribe to changes in the graph
//     graph.subscribe_to_nodes((value: unknown) => {
//       console.log('Graph updated');
//       console.log(value);
//     });

//     graph.run();
//   }, []);

//   return null;
// }
