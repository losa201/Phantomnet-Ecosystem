
import { CustomObjectsApi, AppsV1Api } from '@kubernetes/client-node';
import { getCPUUsageForDeployment } from '../metrics/prometheus';

export async function reconcileAutopilotPolicies(customApi: CustomObjectsApi, appsApi: AppsV1Api) {
  const group = 'phantomnet.io', version = 'v1alpha1', plural = 'autopilotpolicies', ns = 'phantomnet';

  const { body } = await customApi.listNamespacedCustomObject(group, version, ns, plural);
  for (const policy of (body as any).items) {
    const match = policy.spec.matchLabels;
    const threshold = policy.spec.threshold;
    const action = policy.spec.action;

    const selector = Object.entries(match).map(([k, v]) => `${k}=${v}`).join(',');
    const deployments = await appsApi.listNamespacedDeployment(ns, undefined, undefined, undefined, undefined, selector);

    for (const d of deployments.body.items) {
      const cpuUsage = await getCPUUsageForDeployment(d.metadata!.name!, ns);
      console.log(`[Autopilot] ${d.metadata!.name!} CPU usage: ${cpuUsage}`);

      if (cpuUsage > threshold && action === 'scale_up') {
        const newReplicas = (d.spec?.replicas || 1) + 1;
        d.spec!.replicas = newReplicas;
        await appsApi.replaceNamespacedDeployment(d.metadata!.name!, ns, d);
        console.log(`[Autopilot] Scaled ${d.metadata!.name!} to ${newReplicas} replicas.`);
      }
    }
  }
}
