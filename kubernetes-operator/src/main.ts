import { reconcileAutopilotPolicies } from './controllers/autopilot';
import { KubeConfig, AppsV1Api, CustomObjectsApi } from '@kubernetes/client-node';
import express from 'express';
import { applyZeroTrustPolicies } from './security/policy';
import { reconcilePhantomnetNode } from './controllers/node';

const kc = new KubeConfig();
kc.loadFromDefault();
const k8sAppsApi = kc.makeApiClient(AppsV1Api);
const customObjectsApi = kc.makeApiClient(CustomObjectsApi);

const app = express();
app.get('/healthz', (_, res) => res.send('OK'));

async function main() {
  console.log('[PhantomNet Operator] Starting controller...');
  await applyZeroTrustPolicies(k8sAppsApi);
  await reconcilePhantomnetNode(customObjectsApi, k8sAppsApi);
  app.listen(3000, () => console.log('[PhantomNet Operator] Listening on port 3000'));
}

main().catch(err => {
  console.error('[PhantomNet Operator] Fatal error:', err);
  process.exit(1);
});

// ⏱️ Autopilot scheduler
setInterval(() => {
  reconcileAutopilotPolicies(customObjectsApi, k8sAppsApi).catch(console.error);
}, 15000);
