<<<<<<< HEAD
=======
import ZkTangleGraph from './components/ZkTangleGraph';
import RecentVotes from './components/RecentVotes';
import AutopilotPolicyViewer from './components/AutopilotPolicyViewer';
>>>>>>> 1ca2900 (🧠 Initial refined push — 2025-05-28 13:28:07)
import React from 'react';
import { usePhantomNetStream } from './hooks/usePhantomNetStream';

export default function App() {
<<<<<<< HEAD
=======
  // 📡 Autopilot viewer rendered below
>>>>>>> 1ca2900 (🧠 Initial refined push — 2025-05-28 13:28:07)
  const nodes = usePhantomNetStream();
  return (
    <div className="min-h-screen bg-gray-950 text-white p-4">
      <h1 className="text-3xl font-bold mb-6">PhantomNet Nodes</h1>
      <ul>
        {nodes.map((node: any) => (
          <li key={node.name} className="py-2 border-b border-gray-700">
            <span className="font-mono">{node.name}</span> — {node.status}
          </li>
        ))}
      </ul>
<<<<<<< HEAD
    </div>
=======
      <AutopilotPolicyViewer />
</div>
>>>>>>> 1ca2900 (🧠 Initial refined push — 2025-05-28 13:28:07)
  );
}