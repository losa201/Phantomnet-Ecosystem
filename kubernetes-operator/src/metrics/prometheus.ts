
import axios from 'axios';

const PROM_URL = process.env.PROMETHEUS_URL || 'http://prometheus.phantomnet.svc:9090';

export async function getCPUUsageForDeployment(deployment: string, namespace: string): Promise<number> {
  const query = `avg(rate(container_cpu_usage_seconds_total{namespace="${namespace}",pod=~"${deployment}.*"}[2m]))`;
  const res = await axios.get(`${PROM_URL}/api/v1/query`, { params: { query } });

  const value = res.data?.data?.result?.[0]?.value?.[1];
  return value ? parseFloat(value) : 0;
}
