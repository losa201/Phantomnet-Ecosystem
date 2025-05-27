import { useEffect, useState } from 'react';

export function usePhantomNetStream() {
  const [nodes, setNodes] = useState([]);
  useEffect(() => {
    const ws = new WebSocket('ws://localhost:3001/stream');
    ws.onmessage = e => setNodes(JSON.parse(e.data));
    return () => ws.close();
  }, []);
  return nodes;
}