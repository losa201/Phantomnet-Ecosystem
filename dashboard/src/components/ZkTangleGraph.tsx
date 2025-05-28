import React, { useEffect, useState, useRef } from 'react';
import cytoscape from 'cytoscape';

export default function ZkTangleGraph() {
  const containerRef = useRef(null);
  const [graph, setGraph] = useState<any>(null);

  useEffect(() => {
    fetch('/api/tangle')
      .then(res => res.json())
      .then(data => {
        const nodes = data.map((node: any) => ({
          data: { id: node.id, label: node.kind },
        }));

        const edges = data.flatMap((node: any) =>
          node.parents.map((parent: string) => ({
            data: { source: parent, target: node.id },
          }))
        );

        const cy = cytoscape({
          container: containerRef.current,
          elements: [...nodes, ...edges],
          style: [
            {
              selector: 'node',
              style: {
                'label': 'data(label)',
                'background-color': '#4ADE80',
                'color': '#000',
                'text-valign': 'center',
                'text-halign': 'center',
                'font-size': '10px',
              }
            },
            {
              selector: 'edge',
              style: {
                'width': 2,
                'line-color': '#9CA3AF',
                'target-arrow-color': '#9CA3AF',
                'target-arrow-shape': 'triangle',
                'curve-style': 'bezier',
              }
            }
          ],
          layout: {
            name: 'breadthfirst',
            directed: true,
            padding: 10,
            spacingFactor: 1.75,
          }
        });

        setGraph(cy);
      })
      .catch(console.error);
  }, []);

  return (
    <div className="rounded-xl border shadow mt-6 p-2">
      <h2 className="text-xl font-bold mb-2">zkTangle DAG</h2>
      <div ref={containerRef} className="w-full h-[500px]" />
    </div>
  );
}