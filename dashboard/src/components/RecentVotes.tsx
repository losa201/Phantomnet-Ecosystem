import React, { useEffect, useState } from 'react';

export default function RecentVotes() {
  const [votes, setVotes] = useState<any[]>([]);

  useEffect(() => {
    fetch('/api/votes')
      .then(res => res.json())
      .then(data => setVotes(data.reverse()))
      .catch(console.error);
  }, []);

  return (
    <div className="rounded-xl border p-4 shadow mt-6">
      <h2 className="text-xl font-bold mb-4">Recent zkVotes</h2>
      <ul className="space-y-2 max-h-[400px] overflow-y-auto">
        {votes.map((vote, idx) => (
          <li key={idx} className={\`p-2 rounded \${vote.valid ? 'bg-green-100' : 'bg-red-100'}\`}>
            <div className="text-xs text-gray-600">{vote.ts}</div>
            <div className="text-sm break-all">Commitment: <strong>{vote.commitment}</strong></div>
            <div className={\`font-bold \${vote.valid ? 'text-green-700' : 'text-red-700'}\`}>
              {vote.valid ? '✔️ Valid Proof' : '❌ Invalid'}
            </div>
          </li>
        ))}
      </ul>
    </div>
  );
}