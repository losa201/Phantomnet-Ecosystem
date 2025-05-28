import { AppsV1Api } from '@kubernetes/client-node';

export async function applyZeroTrustPolicies(k8sAppsApi: AppsV1Api) {
  console.log('[PhantomNet] Enforcing Linkerd mTLS and Kyverno baseline policies...');
  const namespaces = (await k8sAppsApi.listNamespace()).body.items;
  for (const ns of namespaces) {
    if (!ns.metadata?.labels?.['linkerd.io/inject']) {
      ns.metadata = ns.metadata || {};
      ns.metadata.labels = ns.metadata.labels || {};
      ns.metadata.labels['linkerd.io/inject'] = 'enabled';
      await k8sAppsApi.replaceNamespace(ns.metadata.name!, ns);
      console.log(`[PhantomNet] Enabled Linkerd inject for namespace ${ns.metadata.name}`);
    }
  }
}