
import React, { useEffect, useState } from 'react';

export default function AutopilotPolicyViewer() {
  const [policies, setPolicies] = useState([]);

  useEffect(() => {
    fetch('/api/autopilotpolicies')
      .then(res => res.json())
      .then(data => setPolicies(data.items || []))
      .catch(err => console.error('Failed to fetch policies:', err));
  }, []);

  return (
    <div className="rounded-xl border p-4 shadow">
      <h2 className="text-xl font-bold mb-4">Autopilot Policies</h2>
      <ul className="space-y-2">
        {policies.map((p: any) => (
          <li key={p.metadata.name} className="bg-gray-100 p-2 rounded">
            <div className="font-semibold">{p.metadata.name}</div>
            <div className="text-sm text-gray-700">
              Match: {JSON.stringify(p.spec.matchLabels)}<br/>
              Action: {p.spec.action} when CPU &gt; {p.spec.threshold}
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}
